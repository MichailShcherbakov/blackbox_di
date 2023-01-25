use proc_macro2::TokenStream as TokenStream2;

use super::parse::{ImportField, Module, ProviderField};

pub(crate) fn gen_module_compiler(module: &Module) -> TokenStream2 {
    let module_ident = module.get_ident();
    let module_generics = module.get_generics();

    let path_to_lib = &module.attrs.path_to_lib;

    let mut build_related_modules: Vec<TokenStream2> = Vec::new();
    let mut build_providers: Vec<TokenStream2> = Vec::new();

    module.fields.iter().for_each(|field| {
        if field.is::<ImportField>() {
            let module_path = &field.path;

            build_related_modules.push(quote::quote! {
                let run_build = !module_builder.is_module_exists::<#module_path>();

                let mb = module_builder.register_related_module::<#module_path>();

                if run_build {
                    #module_path::__blackbox_build(mb);
                }
            });
        } else if field.is::<ProviderField>() {
            let provider_path = &field.path;

            let field_instance = field.as_field::<ProviderField>().expect(
                format!(
                    "Runtime Error: Failed to cast the {} provider field",
                    &field.ident.to_string()
                )
                .as_str(),
            );

            let token = &field_instance.token;
            let should_be_exported = if field_instance.should_be_exported {
                quote::quote! {
                    provider_builder.should_be_exported();
                }
            } else {
                quote::quote! {}
            };

            build_providers.push(quote::quote! {
                let provider_builder =
                    module_builder.register_provider::<#provider_path>(#token);

                #should_be_exported

                #provider_path::__blackbox_build(provider_builder);
            });
        }
    });

    let mut attrs: Vec<TokenStream2> = Vec::new();

    if module.attrs.is_module_global {
        attrs.push(quote::quote! { should_be_global() })
    }

    let register_attrs = if attrs.len() != 0 {
        quote::quote! { module_builder #(.#attrs)*; }
    } else {
        quote::quote! {}
    };

    quote::quote! {
      impl #module_generics #path_to_lib::compiler::ModuleCompiler for #module_generics #module_ident {
        fn __blackbox_build(
            module_builder: #path_to_lib::cell::Ref<#path_to_lib::builder::ModuleBuilder>
        ) {
            #register_attrs

            use #path_to_lib::compiler::ProviderCompiler;
            use #path_to_lib::compiler::ModuleCompiler;

            #(#build_providers)*
            #(#build_related_modules)*
        }
      }
    }
}
