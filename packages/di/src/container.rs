use std::collections::{HashMap, HashSet};

use crate::{
    instance_wrapper::{InstanceToken, InstanceWrapper},
    link_mut::LinkMut,
    module::{Module, ModuleToken},
};

pub struct Container {
    pub global_modules_tokens: HashSet<ModuleToken>,
    pub modules: HashMap<ModuleToken, LinkMut<Module>>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            global_modules_tokens: HashSet::new(),
            modules: HashMap::new(),
        }
    }

    pub fn register_module(&mut self, token: ModuleToken, module: LinkMut<Module>) -> &Self {
        self.modules.insert(token, module.clone());

        return self;
    }

    pub fn register_global_module(&mut self, token: ModuleToken, module: LinkMut<Module>) -> &Self {
        self.register_module(token.clone(), module.clone());
        self.global_modules_tokens.insert(token);
        return self;
    }

    pub fn has_module(&self, token: &ModuleToken) -> bool {
        self.modules.contains_key(token)
    }

    pub fn get_module(&self, token: &ModuleToken) -> Option<LinkMut<Module>> {
        if let Some(module) = self.modules.get(token) {
            Some(module.clone())
        } else {
            None
        }
    }

    pub fn get_modules(&self) -> HashMap<ModuleToken, LinkMut<Module>> {
        self.modules
            .iter()
            .map(|(token, module)| (token.clone(), module.clone()))
            .collect::<HashMap<_, _>>()
    }

    pub fn get_global_modules(&self) -> HashMap<ModuleToken, LinkMut<Module>> {
        self.global_modules_tokens
            .iter()
            .map(|token| (token.clone(), self.get_module(token).unwrap()))
            .collect::<HashMap<_, _>>()
    }

    /// Find provider in modules
    ///
    /// Search steps:
    /// 1. looking for the provider in the module providers
    /// 2. looking for the provider in the global module exported providers
    pub fn get_provider(&self, token: &InstanceToken) -> Option<LinkMut<InstanceWrapper>> {
        for (_module_token, module) in self.modules.iter() {
            if let Some(provider) = module.as_ref().get_provider(&token) {
                return Some(provider);
            }
        }

        for (_module_token, module) in self.get_global_modules().iter() {
            if let Some(provider) = module.as_ref().get_provider(&token) {
                return Some(provider);
            }
        }

        None
    }

    /// Find provider in the module
    ///
    /// Search steps:
    /// 1. looking for the provider in the module providers
    /// 2. looking for the provider in the related modules of the module
    /// 3. looking for the provider in the global module providers
    /// 4. looking for the provider in the related modules of the global module
    pub fn get_provider_in_module(
        &self,
        token: &InstanceToken,
        root_module: LinkMut<Module>,
    ) -> Option<LinkMut<InstanceWrapper>> {
        if let Some(provider) = root_module.as_ref().get_provider(&token) {
            return Some(provider.clone());
        }

        for (_token, module) in root_module.as_ref().get_related_modules().iter() {
            if let Some(provider) = module.as_ref().get_exported_provider(&token) {
                return Some(provider.clone());
            }
        }

        for (_token, global_module) in self.get_global_modules().iter() {
            if let Some(provider) = global_module.as_ref().get_exported_provider(&token) {
                return Some(provider.clone());
            }
        }

        for (_token, global_module) in self.get_global_modules().iter() {
            for (_token, module) in global_module.as_ref().get_related_modules().iter() {
                if let Some(provider) = module.as_ref().get_exported_provider(&token) {
                    return Some(provider.clone());
                }
            }
        }

        None
    }

    pub fn get_modules_sorted_by_distance(&self) -> Vec<LinkMut<Module>> {
        let mut modules = self
            .get_modules()
            .iter()
            .map(|(_token, module)| module.clone())
            .collect::<Vec<LinkMut<Module>>>();

        modules.sort_by(|a, b| {
            b.as_ref()
                .get_distance()
                .partial_cmp(&a.as_ref().get_distance())
                .unwrap()
        });

        return modules;
    }
}
