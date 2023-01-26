use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse::ParseStream, parse::Result, Attribute, Ident, Path, Token};

const BLACKBOX_DI_IDENT: &str = "blackbox_di";
const LOCAL_BLACKBOX_DI_IDENT: &str = "local";

/// Getting another path of the lib.
///                                 
/// Default:
///
/// ```rust
/// // ::blackbox_di::interface
/// #[interface]
/// trait Trait {}
/// ```
///
/// Use local crate:
/// ```rust
/// // crate::interface
/// #[interface]
/// #[blackbox_di(crate)]
/// trait Trait {}
/// ```
///
/// Use global path:
/// ```rust
/// // path::to::blackbox_di::interface
/// #[interface]
/// #[blackbox_di(crate = path::to::blackbox_di)]
/// trait Trait {}
/// ```
///
/// Use local path:
/// ```rust
/// // ::interface
/// #[interface]
/// #[blackbox_di(local)]
/// trait Trait {}
/// ```
///
pub(crate) fn get_path_to_lib(attrs: &mut Vec<Attribute>) -> Result<TokenStream2> {
    let mut path_to_lib: Option<TokenStream2> = None;

    attrs.retain(|attr| {
        if !attr.path.is_ident(BLACKBOX_DI_IDENT) {
            return true;
        }

        if let Ok(path) = attr.parse_args_with(|input: ParseStream| {
            input.parse::<Token![crate]>()?;
            input.parse::<Token![=]>()?;
            input.call(Path::parse_mod_style)
        }) {
            path_to_lib = Some(path.to_token_stream())
        } else if attr
            .parse_args_with(|input: ParseStream| input.parse::<Token![crate]>())
            .is_ok()
        {
            path_to_lib = Some(quote::quote! { crate })
        } else if let Ok(ident) = attr.parse_args_with(|input: ParseStream| input.parse::<Ident>())
        {
            if ident.to_string() == LOCAL_BLACKBOX_DI_IDENT {
                path_to_lib = Some(quote::quote! {})
            }
        }

        return false;
    });

    Ok(path_to_lib.unwrap_or_else(|| quote::quote! { blackbox_di }))
}
