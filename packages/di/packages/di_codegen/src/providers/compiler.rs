use proc_macro2::TokenStream as TokenStream2;

use super::{inject::gen_register_deps, parse::Provider, scope::Scope};

pub(crate) fn gen_provider_compiler(provider: &Provider) -> TokenStream2 {
    let provider_ident = &provider.ident;
    let provider_generics = &provider.generics.clone().unwrap();

    let mut register_deps = gen_register_deps(&provider);

    let path_to_lib = &provider.attrs.path_to_lib;

    let scope = match provider.attrs.scope {
        Scope::Singleton => {
            quote::quote! {
                #path_to_lib::instance_wrapper::Scope::Singleton
            }
        }
        Scope::Transient => {
            quote::quote! {
                #path_to_lib::instance_wrapper::Scope::Transient
            }
        }
    };

    let mut attrs: Vec<TokenStream2> = Vec::new();

    attrs.push(quote::quote! { set_scope(#scope) });
    attrs.append(&mut register_deps);

    let register_attrs = if attrs.len() != 0 {
        quote::quote! { provider_builder #(.#attrs)*; }
    } else {
        quote::quote! {}
    };

    quote::quote! {
      impl #provider_generics #path_to_lib::compiler::ProviderCompiler for #provider_ident #provider_generics {
        fn __blackbox_build(
            provider_builder: #path_to_lib::cell::Ref<#path_to_lib::builder::ProviderBuilder>
        ) {
            #register_attrs
        }
      }

      unsafe impl Send for #provider_ident {}
      unsafe impl Sync for #provider_ident {}

      #[#path_to_lib::implements]
      #[blackbox_di(crate = #path_to_lib)]
      impl #provider_generics #path_to_lib::injectable::IInjectable for #provider_ident #provider_generics {}
    }
}
