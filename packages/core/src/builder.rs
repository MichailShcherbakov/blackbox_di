use std::collections::HashMap;

use blackbox_cast::CastFrom;

use crate::{
    app::BlackBoxApp,
    cell::{Ref, RefMut},
    container::Container,
    factory::Factory,
    injectable::IInjectable,
    instance_wrapper::{InstanceToken, InstanceWrapper, Scope, STATIC_CONTEXT},
    module::{Module, ModuleDistance, ModuleToken},
    tokens::get_token,
};

pub type FactoryFn = Box<fn() -> Ref<dyn IInjectable>>;
pub type DepInitFn = Box<dyn Fn(Ref<dyn IInjectable>, Ref<dyn IInjectable>) -> ()>; // (self_, dep) -> ()

pub struct ProviderBuilder {
    pub context: RefMut<BuilderContext>,
    pub instance_wrapper: RefMut<InstanceWrapper>,
    pub factory_fn: FactoryFn,
    pub dep_init_fns: RefMut<HashMap<InstanceToken, DepInitFn>>,
}

fn make_factory_fn<T: Factory + CastFrom>() -> FactoryFn {
    Box::new(|| T::__blackbox_create().cast::<dyn IInjectable>().unwrap())
}

impl ProviderBuilder {
    pub fn new<T: Factory + CastFrom>(
        token: InstanceToken,
        host: RefMut<Module>,
        context: RefMut<BuilderContext>,
    ) -> ProviderBuilder {
        let factory_fn = make_factory_fn::<T>();
        let instance_wrapper = RefMut::new(InstanceWrapper::new(token, host.clone()));

        host.as_mut().register_provider(instance_wrapper.clone());

        ProviderBuilder {
            context,
            factory_fn,
            dep_init_fns: RefMut::new(HashMap::new()),
            instance_wrapper,
        }
    }

    pub fn set_scope(&self, scope: Scope) -> &Self {
        self.instance_wrapper.as_mut().set_scope(scope);
        return self;
    }

    pub fn should_be_exported(&self) -> &Self {
        let token = self.instance_wrapper.as_ref().get_token();
        let host = self.instance_wrapper.as_ref().get_host();

        host.as_mut().register_exported_provider(token);

        return self;
    }

    pub fn link_instance_by_token(
        &self,
        token: &InstanceToken,
        self_instance: Ref<dyn IInjectable>,
    ) -> &Self {
        let host = self.instance_wrapper.as_ref().get_host();
        let instance_wrapper = self
            .context
            .as_ref()
            .container
            .as_ref()
            .get_provider_in_module(token, host.clone())
            .expect(
                format!(
                    "The {} provider in the {} module was not found",
                    token,
                    &host.as_ref().get_token(),
                )
                .as_ref(),
            );

        let scope = instance_wrapper.as_ref().get_scope();

        let dep_init_fns = self.dep_init_fns.as_ref();
        let dep_init_fn = dep_init_fns.get(token).unwrap();

        match scope {
            Scope::Transient => {
                let context_id = STATIC_CONTEXT.to_string();
                let inquirer_id = self.instance_wrapper.as_ref().get_id();

                let dep_instance = instance_wrapper
                    .as_mut()
                    .get_instance_by_inquirer_id(&inquirer_id, &context_id)
                    .expect(
                        format!(
                            "The {} instance of the {} inquirer was not found",
                            &token, &inquirer_id,
                        )
                        .as_ref(),
                    );

                (dep_init_fn)(self_instance, dep_instance);

                return self;
            }
            Scope::Singleton => {
                let dep_instance = instance_wrapper
                    .as_mut()
                    .get_instance()
                    .expect(format!("The {} static instance was not found", &token,).as_ref());

                (dep_init_fn)(self_instance, dep_instance);

                return self;
            }
            _ => {
                panic!("Unknown provider scope");
            }
        }
    }

    pub fn register_instance_by_token(&self, token: &InstanceToken) -> &Self {
        let host = self.instance_wrapper.as_ref().get_host();
        let instance_module = self
            .context
            .as_ref()
            .container
            .as_ref()
            .get_module_by_provider(token, host.clone())
            .expect(
                format!(
                    "The {} provider in the {} module was not found",
                    token,
                    &host.as_ref().get_token(),
                )
                .as_ref(),
            );

        let module_builder = self
            .context
            .as_ref()
            .modules
            .get(&instance_module.as_ref().get_token())
            .unwrap()
            .clone();

        let provider_builder = module_builder
            .as_ref()
            .providers
            .as_ref()
            .get(token)
            .unwrap()
            .clone();

        let instance_wrapper = provider_builder.instance_wrapper.clone();

        let context_id = STATIC_CONTEXT.to_string();
        let inquirer_id = self.instance_wrapper.as_ref().get_id();

        let scope = provider_builder.instance_wrapper.as_ref().get_scope();

        match scope {
            Scope::Transient => {
                if instance_wrapper
                    .as_ref()
                    .has_instance_by_inquirer_id(&inquirer_id, &context_id)
                {
                    return self;
                }

                let instance = (provider_builder.factory_fn)();

                instance_wrapper.as_mut().set_instance_by_inquirer_id(
                    inquirer_id,
                    context_id,
                    instance.clone(),
                );

                return self;
            }
            Scope::Singleton => {
                if instance_wrapper.as_ref().has_instance() {
                    return self;
                }

                let instance = (provider_builder.factory_fn)();

                instance_wrapper.as_mut().set_instance(instance.clone());

                return self;
            }
            _ => {
                panic!("Unknown provider scope");
            }
        }
    }

    pub fn register_dependency<TSelf: Factory + CastFrom, TDep: ?Sized + CastFrom>(
        &self,
        token: String,
        dep_ref_fn: fn(Ref<TSelf>) -> Ref<TDep>,
    ) -> &Self {
        self.dep_init_fns.as_mut().insert(
            token,
            Box::new(
                move |self_: Ref<dyn IInjectable>, dep: Ref<dyn IInjectable>| {
                    (dep_ref_fn)(self_.cast::<TSelf>().unwrap())
                        .__init(dep.cast::<TDep>().unwrap());
                },
            ),
        );

        return self;
    }
}

pub struct ModuleBuilder {
    pub context: RefMut<BuilderContext>,
    pub module: RefMut<Module>,
    pub providers: RefMut<HashMap<InstanceToken, Ref<ProviderBuilder>>>,
}

impl ModuleBuilder {
    pub fn new<T>(context: RefMut<BuilderContext>) -> ModuleBuilder {
        let token = get_token::<T>();
        let module = RefMut::new(Module::new(token.clone()));

        context
            .as_ref()
            .container
            .as_mut()
            .register_module(token, module.clone());

        ModuleBuilder {
            context,
            module,
            providers: RefMut::new(HashMap::new()),
        }
    }

    pub fn should_be_global(&self) -> &Self {
        self.context
            .as_ref()
            .container
            .as_mut()
            .register_global_module_by_token(self.module.as_ref().get_token());

        return self;
    }

    pub fn set_distance(&self, distance: ModuleDistance) -> &Self {
        self.module.as_mut().set_distance(distance);

        return self;
    }

    pub fn register_related_module<T>(&self) -> Ref<ModuleBuilder> {
        let module_builder = if self.is_module_exists::<T>() {
            self.context
                .as_ref()
                .modules
                .get(&get_token::<T>())
                .unwrap()
                .clone()
        } else {
            let module_builder = Ref::new(ModuleBuilder::new::<T>(self.context.clone()));

            module_builder.set_distance(self.module.as_ref().get_distance() + 1);

            self.context.as_mut().modules.insert(
                module_builder.module.as_ref().get_token(),
                module_builder.clone(),
            );

            module_builder
        };

        self.module
            .as_mut()
            .register_related_module(module_builder.module.clone());

        return module_builder;
    }

    pub fn register_provider<T: Factory + CastFrom>(
        &self,
        token: InstanceToken,
    ) -> Ref<ProviderBuilder> {
        let provider_builder = Ref::new(ProviderBuilder::new::<T>(
            token.clone(),
            self.module.clone(),
            self.context.clone(),
        ));

        self.providers
            .as_mut()
            .insert(token, provider_builder.clone());

        return provider_builder;
    }

    pub fn get_context(&self) -> RefMut<BuilderContext> {
        self.context.clone()
    }

    pub fn is_module_exists<T>(&self) -> bool {
        let token = get_token::<T>();

        self.context.as_ref().modules.contains_key(&token)
    }
}

pub struct BuilderContext {
    pub container: RefMut<Container>,
    pub modules: HashMap<ModuleToken, Ref<ModuleBuilder>>,
}

impl BuilderContext {
    pub fn new() -> BuilderContext {
        BuilderContext {
            modules: HashMap::new(),
            container: RefMut::new(Container::new()),
        }
    }
}

pub struct Builder {
    context: RefMut<BuilderContext>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            context: RefMut::new(BuilderContext::new()),
        }
    }

    pub fn register_module<T>(&self) -> Ref<ModuleBuilder> {
        let module_builder = Ref::new(ModuleBuilder::new::<T>(self.context.clone()));

        self.context.as_mut().modules.insert(
            module_builder.module.as_ref().get_token(),
            module_builder.clone(),
        );

        return module_builder;
    }

    pub fn get_modules(&self) -> HashMap<ModuleToken, Ref<ModuleBuilder>> {
        self.context.as_ref().modules.clone()
    }

    pub fn build(&self) -> Ref<BlackBoxApp> {
        Ref::new(BlackBoxApp::new(self.context.as_ref().container.clone()))
    }
}
