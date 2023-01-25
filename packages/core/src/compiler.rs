use std::collections::HashSet;

use blackbox_cast::*;

use crate::{
    builder::{ModuleBuilder, ProviderBuilder},
    container::Container,
    factory::Factory,
    injectable::IInjectable,
    instance_wrapper::{ContextId, InquirerId, InstanceToken, Scope},
    module::{Module, ModuleToken},
    reference::Ref,
    reference_mut::RefMut,
};

pub struct ProviderCompilerContext {
    pub inquirer_id: InquirerId,
    pub context_id: ContextId,
}
impl ProviderCompilerContext {
    pub fn new(context_id: ContextId, inquirer_id: InquirerId) -> ProviderCompilerContext {
        ProviderCompilerContext {
            context_id,
            inquirer_id,
        }
    }
}

pub trait ProviderCompiler {
    fn __blackbox_build(provider_builder: Ref<ProviderBuilder>);
}

pub struct ModuleCompilerContext {
    pub stack: HashSet<ModuleToken>,
    pub current_depth: u32,
}

impl ModuleCompilerContext {
    pub fn new() -> ModuleCompilerContext {
        ModuleCompilerContext {
            stack: HashSet::new(),
            current_depth: 0,
        }
    }
}

pub trait ModuleCompiler {
    fn __blackbox_build(module_builder: Ref<ModuleBuilder>);
}

pub fn make_instance<T: Factory + CastFrom>(
    token: InstanceToken,
    module: RefMut<Module>,
    container: RefMut<Container>,
    context: RefMut<ProviderCompilerContext>,
) -> Ref<dyn IInjectable> {
    let instance_wrapper = container
        .as_ref()
        .get_provider_in_module(&token, module.clone())
        .expect(
            format!(
                "The {} provider in the {} module was not found",
                &token,
                &module.as_ref().get_token(),
            )
            .as_ref(),
        );

    let scope = instance_wrapper.as_ref().get_scope();

    match scope {
        Scope::Transient => {
            let context_id = context.as_ref().context_id.clone();
            let inquirer_id = context.as_ref().inquirer_id.clone();

            if let Some(instance) = instance_wrapper
                .as_ref()
                .get_instance_by_inquirer_id(&inquirer_id, &context_id)
            {
                return instance;
            }

            let instance = T::__blackbox_create().cast::<dyn IInjectable>().unwrap();

            instance_wrapper.as_mut().set_instance_by_inquirer_id(
                inquirer_id,
                context_id,
                instance.clone(),
            );

            return instance;
        }
        Scope::Singleton => {
            if let Some(instance) = instance_wrapper.as_ref().get_instance() {
                return instance;
            }

            let instance = T::__blackbox_create().cast::<dyn IInjectable>().unwrap();

            instance_wrapper.as_mut().set_instance(instance.clone());

            return instance;
        }
        _ => {
            panic!("Unknown provider scope");
        }
    }
}

pub fn get_instance<T: Factory + CastFrom>(
    token: InstanceToken,
    module: RefMut<Module>,
    container: RefMut<Container>,
    context: RefMut<ProviderCompilerContext>,
) -> Ref<T> {
    let instance_wrapper = container
        .as_ref()
        .get_provider_in_module(&token, module.clone())
        .expect(
            format!(
                "The {} provider in the {} module was not found",
                &token,
                &module.as_ref().get_token(),
            )
            .as_ref(),
        );

    let scope = instance_wrapper.as_ref().get_scope();

    match scope {
        Scope::Transient => {
            let context_id = context.as_ref().context_id.clone();
            let inquirer_id = context.as_ref().inquirer_id.clone();

            return instance_wrapper
                .as_mut()
                .get_instance_by_inquirer_id(&inquirer_id, &context_id)
                .expect(
                    format!(
                        "The {} instance of the {} inquirer was not found",
                        &token, &inquirer_id,
                    )
                    .as_ref(),
                )
                .cast::<T>()
                .unwrap();
        }
        Scope::Singleton => {
            return instance_wrapper
                .as_mut()
                .get_instance()
                .expect(format!("The {} static instance was not found", &token,).as_ref())
                .cast::<T>()
                .unwrap();
        }
        _ => {
            panic!("Unknown provider scope");
        }
    }
}

// 1. (build) Modules, Instance Wrappers (Host, Id, Token, Scope)
// 2. (init) Creation of the instances (Static/Transient)
// 3. (link) Linking refs
