use std::collections::HashMap;

use cast::CastFrom;

use crate::{
    builder::Builder,
    cell::RefMut,
    compiler::ModuleCompiler,
    container::Container,
    factory::Factory,
    instance_wrapper::{InstanceToken, InstanceWrapper, Scope},
    module::{Module, ModuleId},
    reference::Ref,
    tokens::get_token,
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
        BlackBoxApp {
            container: container.clone(),
            instance_links_host: RefMut::new(InstanceLinksHost::new(container)),
        }
    }

    pub fn init(&self) -> &Self {
        self.instance_links_host.as_mut().init();

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
            let scope = wrapper.as_ref().get_scope();

            if scope == Scope::Transient || scope == Scope::ContextDependent {
                return Err(format!("InstanceLinksHost: {} is marked as a scoped provider. Context depend and transient-scoped providers can't be used in combination with `get()` method.", token));
            }

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
}

pub fn build<TModule: ModuleCompiler>() -> Ref<BlackBoxApp> {
    let builder = RefMut::new(Builder::new());

    let root_module_builder = builder.as_mut().register_module::<TModule>();
    TModule::__blackbox_build(root_module_builder);

    init(builder.clone());
    link(builder.clone());

    return builder.as_ref().build();
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
