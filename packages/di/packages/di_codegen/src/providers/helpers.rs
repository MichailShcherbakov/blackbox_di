use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::{Ident, ItemImpl, Type};

pub fn gen_dep_ident(uident: usize) -> Ident {
    Ident::new(format!("dep_{}", uident).as_str(), Span::call_site())
}

pub fn get_provider_ident_from_impl_block(impl_block: &ItemImpl) -> TokenStream2 {
    if let Type::Path(provider_path) = &*impl_block.self_ty {
        provider_path
            .path
            .segments
            .first()
            .unwrap()
            .ident
            .to_token_stream()
    } else {
        panic!("Failed to get a provider name");
    }
}
