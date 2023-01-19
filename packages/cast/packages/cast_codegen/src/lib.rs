mod helpers;
mod path;

use helpers::{gen_fn_name, get_trait_path};
use path::get_path_to_lib;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn cast(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut impl_block =
        syn::parse::<syn::ItemImpl>(item.clone()).expect("#[cast] can be used only on impl blocks");

    let path_to_lib = get_path_to_lib(&mut impl_block.attrs).expect("Can't get the lib path");

    let mut result = TokenStream2::from(impl_block.to_token_stream());

    let ty = impl_block.self_ty.to_token_stream();
    let trait_ = get_trait_path(&impl_block)
        .expect("#[cast] can be used only on impl blocks for traits")
        .to_token_stream();

    result.extend(create_caster(&ty, &ty, &path_to_lib));
    result.extend(create_caster(
        &quote::quote! { dyn #trait_ },
        &ty,
        &path_to_lib,
    ));

    return result.into();
}

fn create_caster(
    from: &TokenStream2,
    to: &TokenStream2,
    path_to_lib: &TokenStream2,
) -> TokenStream2 {
    let trait_caster = quote::quote! {
        #path_to_lib::Caster::<#from>::new(
            |from| from.downcast::<#to>().unwrap(),
        )
    };

    let fn_ident = gen_fn_name();

    quote::quote! {
        #[#path_to_lib::linkme::distributed_slice(#path_to_lib::TRAITCASTERS)]
        #[linkme(crate = #path_to_lib::linkme)]
        fn #fn_ident() -> (#path_to_lib::TargetId, #path_to_lib::BoxedTraitCaster) {
            (::std::any::TypeId::of::<#to>(), Box::new(#trait_caster))
        }
    }
}
