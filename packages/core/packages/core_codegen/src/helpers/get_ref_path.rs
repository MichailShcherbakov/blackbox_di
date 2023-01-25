use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{Error, Path};

use super::get_type_arg;

/// Getting a ref path
///
/// Examples:
///
/// Service => Service
/// Ref<Service> => Service
/// di::ref::Ref<Service> => Service
pub(crate) fn get_ref_path(path: &Path) -> Result<TokenStream2, Error> {
    let path_name = path.to_token_stream().to_string();

    return if path_name.starts_with("di :: ref :: Ref <") {
        get_type_arg(&path.segments[1].arguments)
    } else if path_name.starts_with("Ref <") {
        get_type_arg(&path.segments[0].arguments)
    } else {
        Ok(path.to_token_stream())
    };
}
