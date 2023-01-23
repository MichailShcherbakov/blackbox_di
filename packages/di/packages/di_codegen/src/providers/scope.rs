use syn::{
    parse::{ParseStream, Result},
    Attribute, Ident,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    /// The provider can be shared across multiple structure.
    Singleton,
    /// A new private instance of the provider is instantiated for every use
    Transient,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Singleton
    }
}

const SCOPE_IDENT: &str = "scope";
const SINGLETON_IDENT: &str = "Singleton";
const TRANSIENT_IDENT: &str = "Transient";

pub(crate) fn get_scope(attrs: &mut Vec<Attribute>) -> Result<Scope> {
    let mut scope = Scope::default();

    attrs.retain(|attr| {
        if !attr.path.is_ident(SCOPE_IDENT) {
            return true;
        }

        if let Ok(scope_ident) = attr.parse_args_with(|input: ParseStream| input.parse::<Ident>()) {
            match scope_ident.to_string().as_str() {
                SINGLETON_IDENT => {
                    scope = Scope::Singleton;
                }
                TRANSIENT_IDENT => {
                    scope = Scope::Transient;
                }
                _ => {
                    scope = Scope::default();
                }
            }
        }

        return false;
    });

    Ok(scope)
}
