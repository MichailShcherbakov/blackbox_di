use proc_macro2::Span;
use syn::{Ident, ItemImpl, Path};
use uuid::Uuid;

pub fn get_trait_path(impl_block: &ItemImpl) -> Option<&Path> {
    if let Some((_, trait_, _)) = &impl_block.trait_ {
        Some(trait_)
    } else {
        None
    }
}

const FN_PREFIX: &'static str = "__cast_";

pub fn gen_fn_name() -> Ident {
    let uuidv4 = Uuid::new_v4()
        .simple()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_string();

    Ident::new(
        (FN_PREFIX.to_string() + &uuidv4).as_str(),
        Span::call_site(),
    )
}
