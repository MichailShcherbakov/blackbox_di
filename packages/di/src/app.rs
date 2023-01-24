use std::collections::HashMap;

use cast::CastFrom;

use crate::{
    builder::Builder,
    cell::RefMut,
    compiler::ModuleCompiler,
    container::Container,
    events::{OnModuleDestroy, OnModuleInit},
    instance_wrapper::{InstanceToken, InstanceWrapper, Scope},
    module::{Module, ModuleId},
    modules::CoreModule,
    reference::Ref,
    tokens::get_token,
    ILogger, Logger,
};

#[derive(Clone)]
pub struct InstanceLink {
    pub wrapper_ref: RefMut<InstanceWrapper>,
    pub module_id: ModuleId,
}

impl InstanceLink {
    pub fn new(module_id: ModuleId, wrapper_ref: RefMut<InstanceWrapper>) -> InstanceLink {
        InstanceLink {
            module_id,
            wrapper_ref,
        }
    }
}

pub struct InstanceLinksHost {
    container: RefMut<Container>,
    instance_links: HashMap<InstanceToken, Vec<InstanceLink>>,
}

impl InstanceLinksHost {
    pub fn new(container: RefMut<Container>) -> InstanceLinksHost {
        InstanceLinksHost {
            container,
            instance_links: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        let modules = self.container.as_ref().get_modules();

        for (_token, module) in modules {
            let providers = module.as_ref().get_providers();

            for (_token, provider) in providers {
                self.add_link(module.clone(), provider.clone());
            }
        }
    }

    fn add_link(&mut self, module_ref: RefMut<Module>, wrapper_ref: RefMut<InstanceWrapper>) {
        let token = wrapper_ref.as_ref().get_token();

        let instance_link = InstanceLink::new(module_ref.as_ref().get_id(), wrapper_ref.clone());

        if let Some(existing_links) = self.instance_links.get_mut(&token) {
            existing_links.push(instance_link);
        } else {
            self.instance_links.insert(token, vec![instance_link]);
        }
    }

    pub fn get_by_token(&self, token: &InstanceToken) -> Option<Vec<InstanceLink>> {
        if let Some(instance_links) = self.instance_links.get(token) {
            Some(instance_links.clone())
        } else {
            None
        }
    }
}

pub struct BlackBoxApp {
    container: RefMut<Container>,
    instance_links_host: RefMut<InstanceLinksHost>,
}

impl BlackBoxApp {
    pub fn new(container: RefMut<Container>) -> BlackBoxApp {
        let instance_links_host = RefMut::new(InstanceLinksHost::new(container.clone()));

        instance_links_host.as_mut().init();

        BlackBoxApp {
            container,
            instance_links_host,
        }
    }

    pub async fn init(&self) -> &Self {
        self.call_init_hook().await;

        return self;
    }

    /// Retrieves an instance of either injectable, otherwise, throws error.
    pub fn get<TInjectable: CastFrom>(&self) -> Result<Ref<TInjectable>, String> {
        self.get_by_token(&get_token::<TInjectable>())
    }

    /// Retrieves an instance of either injectable by token, otherwise, throws error.
    pub fn get_by_token<TInjectable: ?Sized + CastFrom>(
        &self,
        token: &InstanceToken,
    ) -> Result<Ref<TInjectable>, String> {
        let instance_links = self
            .instance_links_host
            .as_ref()
            .get_by_token(token)
            .expect(format!("InstanceLinksHost: {} provider was not found.", token).as_str());

        for instance_link in instance_links {
            let wrapper = instance_link.wrapper_ref.clone();

            return Ok(wrapper
                .as_ref()
                .get_instance()
                .unwrap()
                .cast::<TInjectable>()
                .unwrap());
        }

        return Err(format!(
            "InstanceLinksHost: {} provider was not found.",
            token
        ));
    }

    async fn call_init_hook(&self) {
        let modules = self.container.as_ref().get_modules_sorted_by_distance();

        for module in modules {
            let providers = module.as_ref().get_providers();

            for (_token, provider) in providers {
                let instances = provider.as_ref().get_instances();

                for instance in instances {
                    if let Ok(provider) = instance.cast::<dyn OnModuleInit>() {
                        provider.as_ref().on_module_init().await;
                    }
                }
            }
        }
    }

    async fn call_destroy_hook(&self) {
        let modules = self.container.as_ref().get_modules_sorted_by_distance();

        for module in modules {
            let providers = module.as_ref().get_providers();

            for (_token, provider) in providers {
                let instances = provider.as_ref().get_instances();

                for instance in instances {
                    if let Ok(provider) = instance.cast::<dyn OnModuleDestroy>() {
                        provider.as_ref().on_module_destroy().await;
                    }
                }
            }
        }
    }

    pub fn use_logger(&self, logger: Ref<dyn ILogger>) {
        let default_logger = self.get::<Logger>().unwrap();

        default_logger.as_ref().register_logger(logger);
        default_logger.as_ref().flush();
    }
}

pub struct BuildParams {
    buffer_logs: bool,
}

impl BuildParams {
    pub fn default() -> BuildParams {
        BuildParams { buffer_logs: false }
    }

    pub fn buffer_logs(mut self) -> Self {
        self.buffer_logs = true;

        return self;
    }
}

pub async fn build<TModule: ModuleCompiler>(params: BuildParams) -> Ref<BlackBoxApp> {
    let builder = RefMut::new(Builder::new());

    let core_module_builder = builder.as_mut().register_module::<CoreModule>();
    CoreModule::__blackbox_build(core_module_builder);

    let root_module_builder = builder.as_mut().register_module::<TModule>();
    TModule::__blackbox_build(root_module_builder);

    init(builder.clone());
    link(builder.clone());

    let raw_container = builder.as_ref().get_raw_container();
    let mut modules = raw_container.as_ref().get_modules_sorted_by_distance();

    modules.reverse();

    let app = builder.as_ref().build();

    let logger = app.get::<Logger>().unwrap();

    if params.buffer_logs {
        logger.attach_buffer();
    }

    for module in modules {
        let token = module.as_ref().get_token();

        // skip core modules
        if token.starts_with("di::") {
            continue;
        }

        logger.info_with_ctx(
            format!("{} dependencies initialized", token).as_str(),
            "ModuleLoader",
        )
    }

    return app;
}

fn init(builder: RefMut<Builder>) {
    let module_builders = builder.as_ref().get_modules();

    for (_token, module_builder) in module_builders {
        let providers = module_builder.providers.as_ref().clone();

        for (token, provider_builder) in providers {
            let dep_tokens = provider_builder.dep_init_fns.clone();

            // static context
            provider_builder.register_instance_by_token(&token);

            for (token, _) in dep_tokens.as_ref().iter() {
                provider_builder.register_instance_by_token(token);
            }
        }
    }
}

fn link(builder: RefMut<Builder>) {
    let module_builders = builder.as_ref().get_modules();

    for (_token, module_builder) in module_builders {
        let providers = module_builder.providers.as_ref().clone();

        for (_token, provider_builder) in providers {
            let dep_tokens = provider_builder.dep_init_fns.clone();

            let instances = provider_builder.instance_wrapper.as_ref().get_instances();

            for instance in instances {
                for (token, _) in dep_tokens.as_ref().iter() {
                    provider_builder.link_instance_by_token(token, instance.clone());
                }
            }
        }
    }
}
