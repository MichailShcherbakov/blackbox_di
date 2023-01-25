use std::iter::FromIterator;

use super::dependency::ProviderDependency;

#[derive(Debug)]
pub struct ProviderDependencyCollection(Vec<ProviderDependency>);

impl FromIterator<ProviderDependency> for ProviderDependencyCollection {
    fn from_iter<T: IntoIterator<Item = ProviderDependency>>(iter: T) -> Self {
        ProviderDependencyCollection(Vec::from_iter(iter))
    }
}

impl From<Vec<ProviderDependency>> for ProviderDependencyCollection {
    fn from(collection: Vec<ProviderDependency>) -> Self {
        ProviderDependencyCollection(collection)
    }
}

impl Into<Vec<ProviderDependency>> for ProviderDependencyCollection {
    fn into(self) -> Vec<ProviderDependency> {
        self.0
    }
}

impl ProviderDependencyCollection {
    pub fn new() -> ProviderDependencyCollection {
        ProviderDependencyCollection(Vec::new())
    }

    pub fn get_injectable_deps(&self) -> Vec<&ProviderDependency> {
        self.0
            .iter()
            .filter(|dependency| dependency.is_injectable())
            .collect::<Vec<_>>()
    }

    pub fn has_uninjectable_deps(&self) -> bool {
        self.0.iter().any(|dependency| !dependency.is_injectable())
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ProviderDependency> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
