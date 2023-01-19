use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{spanned::Spanned, Error, PathArguments};

pub fn get_type_arg(arguments: &PathArguments) -> Result<TokenStream2, Error> {
    if let PathArguments::AngleBracketed(ab) = arguments {
        Ok(ab.args.to_token_stream())
    } else {
        Err(Error::new(arguments.span(), "Unsupported type argument"))
    }
}
