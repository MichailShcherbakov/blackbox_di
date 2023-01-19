use std::collections::HashMap;

use cast::CastFrom;

use crate::{link::Link, link_mut::LinkMut, module::Module};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    /// The provider can be shared across multiple structure.
    Singleton,
    /// A new private instance of the provider is instantiated for every use
    Transient,
    /// Reserved
    ContextDependent,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Singleton
    }
}

static STATIC_CONTEXT: &'static str = "STATIC_CONTEXT";

pub trait IInjectable: CastFrom {}

pub type InstanceWrapperId = String;
pub type InquirerId = InstanceWrapperId;
pub type InstanceToken = String;
pub type ContextId = String;
pub type Instance = Link<dyn IInjectable>;

pub struct InstanceWrapper {
    id: InstanceWrapperId,
    token: InstanceToken,
    host: LinkMut<Module>,
    scope: Scope,
    instance_collection: HashMap<InquirerId, HashMap<ContextId, Instance>>,
}

impl InstanceWrapper {
    pub fn new(token: InstanceToken, host: LinkMut<Module>) -> InstanceWrapper {
        InstanceWrapper {
            id: gen_instance_wrapper_id(),
            token,
            host,
            scope: Scope::default(),
            instance_collection: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> InstanceWrapperId {
        self.id.clone()
    }

    pub fn get_token(&self) -> InstanceToken {
        self.token.clone()
    }

    pub fn get_host(&self) -> LinkMut<Module> {
        self.host.clone()
    }

    pub fn get_scope(&self) -> Scope {
        self.scope.clone()
    }

    pub fn has_instance(&self) -> bool {
        self.get_instance().is_some()
    }

    pub fn set_instance(&mut self, instance: Instance) {
        self.set_instance_by_inquirer_id(
            self.get_id().clone(),
            STATIC_CONTEXT.to_string(),
            instance,
        )
    }

    pub fn get_instance(&self) -> Option<Instance> {
        self.get_instance_by_inquirer_id(&self.get_id(), &STATIC_CONTEXT.to_string())
    }

    pub fn set_instance_by_inquirer_id(
        &mut self,
        inquirer_id: InquirerId,
        context_id: ContextId,
        instance: Instance,
    ) {
        let instances: &mut HashMap<_, _> = self
            .instance_collection
            .entry(inquirer_id)
            .or_insert_with(|| HashMap::new())
            .into();

        instances.insert(context_id, instance);
    }

    pub fn get_instance_by_inquirer_id(
        &self,
        inquirer_id: &InquirerId,
        context_id: &ContextId,
    ) -> Option<Instance> {
        if let Some(instances) = self.instance_collection.get(inquirer_id) {
            if let Some(instance) = instances.get(context_id) {
                Some(instance.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_instance_by_context_id(&mut self, context_id: ContextId, instance: Instance) {
        self.set_instance_by_inquirer_id(self.get_id().clone(), context_id, instance)
    }

    pub fn get_instance_by_context_id(&self, context_id: &ContextId) -> Option<Instance> {
        self.get_instance_by_inquirer_id(&self.get_id(), context_id)
    }
}

fn gen_instance_wrapper_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
