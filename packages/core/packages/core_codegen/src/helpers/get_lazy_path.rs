use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{Error, Path};

use super::get_type_arg;

/// Getting a lazy path
///
/// Examples:
///
/// Module => Module
/// Lazy<Module> => Module
/// blackbox_di::lazy::Lazy<Module> => Module
pub(crate) fn get_lazy_path(
    path: &Path,
    path_to_lib: &TokenStream2,
) -> Result<TokenStream2, Error> {
    let path_name = path.to_token_stream().to_string();

    return if path_name
        .starts_with(format!("{} :: lazy :: Lazy <", path_to_lib.to_string()).as_str())
    {
        get_type_arg(&path.segments[1].arguments)
    } else if path_name.starts_with("Lazy <") {
        get_type_arg(&path.segments[0].arguments)
    } else {
        Ok(path.to_token_stream())
    };
}
