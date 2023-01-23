mod helpers;
mod interfaces;
mod modules;
mod providers;

use helpers::get_path_to_lib;
use interfaces::parse_interface;
use modules::{gen_module_compiler, parse_module};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use providers::{gen_factory_code, gen_provider_compiler, parse_provider};
use quote::ToTokens;
use syn::{parse, ItemFn};

#[proc_macro_attribute]
pub fn injectable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let provider = parse_provider(item).unwrap();

    let mut result = provider.to_token_stream();

    result.extend(gen_provider_compiler(&provider));

    if !provider.needs_factory() {
        result.extend(gen_factory_code(&provider))
    }

    result.into()
}

#[proc_macro_attribute]
pub fn module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let module = parse_module(item).unwrap();

    let mut result = module.to_token_stream();

    result.extend(gen_module_compiler(&module));

    result.into()
}

#[proc_macro_attribute]
pub fn interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let interface = parse_interface(item).unwrap();

    interface.to_token_stream().into()
}

#[proc_macro_attribute]
pub fn implements(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let provider = parse_provider(item).expect("#[implements] can be used only on impl blocks");

    let mut result = TokenStream2::new();

    if provider.has_interface() {
        let item = provider.to_token_stream();
        let path_to_lib = &provider.attrs.path_to_lib;

        result.extend(quote::quote! {
            #[#path_to_lib::async_trait::async_trait]
            #[#path_to_lib::cast::cast]
            #[cast(crate = #path_to_lib::cast)]
            #item
        });

        return result.into();
    }

    result.extend(provider.to_token_stream());

    if provider.has_factory() {
        result.extend(gen_factory_code(&provider));
    };

    return result.into();
}

#[proc_macro_attribute]
pub fn launch(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_ = parse::<ItemFn>(item).expect("#[launch] can be used only on fn blocks");

    let path_to_lib = get_path_to_lib(&mut fn_.attrs).unwrap();

    let mut result = TokenStream2::from(fn_.to_token_stream());

    let block_ = fn_.block;

    result.extend(quote::quote! {
         fn main() {
            #path_to_lib::tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async #block_)
        }
    });

    return result.into();
}
