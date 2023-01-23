use proc_macro2::TokenStream as TokenStream2;
use syn::{Ident, Path};

#[derive(Debug)]
pub struct Inject {
    pub token: TokenStream2,
}

impl Inject {
    pub fn new(token: TokenStream2) -> Inject {
        Inject { token }
    }
}

#[derive(Debug)]
pub struct ProviderDependency {
    pub ident: Ident,
    pub path: Path,
    pub inject: Option<Inject>,
}

impl ProviderDependency {
    pub fn new(ident: Ident, path: Path, inject: Option<Inject>) -> ProviderDependency {
        ProviderDependency {
            ident,
            path,
            inject,
        }
    }

    pub fn is_injectable(&self) -> bool {
        self.inject.is_some()
    }
}
