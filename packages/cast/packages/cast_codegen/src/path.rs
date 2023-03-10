use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse::ParseStream, parse::Result, Attribute, Ident, Path, Token};

const CAST_IDENT: &str = "cast";
const LOCAL_CAST_IDENT: &str = "local";

/// Getting another path of the lib.
///                                 
/// Default:
///
/// ```rust
/// // ::cast::cast
/// #[cast]
/// impl Trait for Structure {}
/// ```
///
/// Use local crate:
/// ```rust
/// // crate::cast
/// #[cast]
/// #[cast(crate)]
/// impl Trait for Structure {}
/// ```
///
/// Use global path:
/// ```rust
/// // path::to::cast::cast
/// #[cast]
/// #[cast(crate = path::to::cast)]
/// impl Trait for Structure {}
/// ```
///
/// Use local path:
/// ```rust
/// // ::cast
/// #[cast]
/// #[cast(local]
/// impl Trait for Structure {}
/// ```
pub(crate) fn get_path_to_lib(attrs: &mut Vec<Attribute>) -> Result<TokenStream2> {
    let mut path_to_lib: Option<TokenStream2> = None;

    attrs.retain(|attr| {
        if !attr.path.is_ident(CAST_IDENT) {
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
            if ident.to_string() == LOCAL_CAST_IDENT {
                path_to_lib = Some(quote::quote! {})
            }
        }

        return false;
    });

    Ok(path_to_lib.unwrap_or_else(|| quote::quote! { blackbox_cast }))
}
