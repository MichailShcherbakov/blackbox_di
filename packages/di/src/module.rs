use std::collections::{HashMap, HashSet};

use crate::{
    instance_wrapper::{InstanceToken, InstanceWrapper},
    reference_mut::RefMut,
};

pub type ModuleId = String;
pub type ModuleToken = String;

pub struct Module {
    id: ModuleId,
    token: ModuleToken,
    distance: u32,
    related_modules: HashMap<ModuleToken, RefMut<Module>>,
    providers: HashMap<InstanceToken, RefMut<InstanceWrapper>>,
    exported_providers_tokens: HashSet<InstanceToken>,
}

impl Module {
    pub fn new(token: String) -> Module {
        Module {
            id: gen_module_id(),
            token,
            distance: 0,
            related_modules: HashMap::new(),
            providers: HashMap::new(),
            exported_providers_tokens: HashSet::new(),
        }
    }

    pub fn get_id(&self) -> ModuleId {
        self.id.clone()
    }

    pub fn get_token(&self) -> ModuleToken {
        self.token.clone()
    }

    pub fn get_distance(&self) -> u32 {
        self.distance.clone()
    }

    pub fn set_distance(&mut self, value: u32) {
        self.distance = value;
    }

    pub fn register_related_module(&mut self, module: RefMut<Module>) -> &Self {
        self.related_modules
            .insert(module.as_ref().token.clone(), module.clone());

        return self;
    }

    pub fn register_provider(&mut self, provider: RefMut<InstanceWrapper>) -> &Self {
        self.providers
            .insert(provider.as_ref().get_token().clone(), provider.clone());

        return self;
    }

    pub fn register_exported_provider(&mut self, token: InstanceToken) -> &Self {
        if !self.providers.contains_key(&token) {
            panic!(
                "The {} provider was not found in the {} module",
                &token, &self.token
            );
        }

        self.exported_providers_tokens.insert(token);

        return self;
    }

    pub fn get_related_modules(&self) -> HashMap<String, RefMut<Module>> {
        self.related_modules
            .iter()
            .map(|(token, module)| (token.clone(), module.clone()))
            .collect::<HashMap<_, _>>()
    }

    pub fn get_providers(&self) -> HashMap<String, RefMut<InstanceWrapper>> {
        self.providers
            .iter()
            .map(|(token, provider)| (token.clone(), provider.clone()))
            .collect::<HashMap<_, _>>()
    }

    pub fn get_exported_providers(&self) -> HashMap<String, RefMut<InstanceWrapper>> {
        self.exported_providers_tokens
            .iter()
            .map(|token| (token.clone(), self.get_provider(token).unwrap()))
            .collect::<HashMap<_, _>>()
    }

    pub fn get_related_module(&self, token: &String) -> Option<RefMut<Module>> {
        if let Some(module) = self.related_modules.get(token) {
            Some(module.clone())
        } else {
            None
        }
    }

    pub fn get_provider(&self, token: &String) -> Option<RefMut<InstanceWrapper>> {
        if let Some(provider) = self.providers.get(token) {
            Some(provider.clone())
        } else {
            None
        }
    }

    pub fn get_exported_provider(&self, token: &String) -> Option<RefMut<InstanceWrapper>> {
        if let Some(found_token) = self.exported_providers_tokens.get(token) {
            self.get_provider(found_token)
        } else {
            None
        }
    }
}

fn gen_module_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
