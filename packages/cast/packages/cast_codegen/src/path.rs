use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse::ParseStream, parse::Result, Attribute, Ident, Path, Token};

static CAST_IDENT: &'static str = "cast";
static LOCAL_CAST_IDENT: &'static str = "local";

/// Getting another path of the lib.
///                                 
/// Default: `::cast::cast`
///
/// ```rust
/// #[cast]
/// trait Trait for Structure {}
/// ```
///
/// Use local crate: `crate::cast`
/// ```rust
/// #[cast]
/// #[cast(crate)]
/// trait Trait for Structure {}
/// ```
///
/// Use global path: `path::to::cast`
/// ```rust
/// #[cast]
/// #[cast(crate = path::to::cast)]
/// trait Trait for Structure {}
/// ```
///
/// Use local path: `::cast`
/// ```rust
/// #[cast]
/// #[cast(local]
/// trait Trait for Structure {}
/// ```
///
pub fn get_path_to_lib(attrs: &mut Vec<Attribute>) -> Result<TokenStream2> {
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

    Ok(path_to_lib.unwrap_or_else(|| quote::quote! { cast }))
}
