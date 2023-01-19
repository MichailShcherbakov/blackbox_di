mod helpers;
mod parse;

pub(crate) use parse::*;

use proc_macro2::TokenStream as TokenStream2;

pub(crate) fn gen_module_compiler(module: &Module) -> TokenStream2 {
    let module_ident = module.get_ident();
    let module_generics = module.get_generics();

    let path_to_lib = module.get_attrs().get_path_to_lib();

    let mut compiler_related_modules: Vec<TokenStream2> = Vec::new();
    let mut init_related_modules: Vec<TokenStream2> = Vec::new();

    let mut compiler_providers: Vec<TokenStream2> = Vec::new();
    let mut init_providers: Vec<TokenStream2> = Vec::new();

    module.get_fields().iter().for_each(|field| {
        if field.is::<ImportField>() {
            let module_path = field.get_path();

            compiler_related_modules.push(quote::quote! {
                context.as_mut().current_depth = module.as_ref().get_distance() + 1;

                let related_module = #module_path::__compile(container.clone(), context.clone());
                module.as_mut().register_related_module(related_module);
            });

            init_related_modules.push(quote::quote! {
                #module_path::__init(container.clone(), context.clone());
            });
        } else if field.is::<ProviderField>() {
            let provider_path = field.get_path();

            let field_instance = field.as_field::<ProviderField>().expect(
                format!(
                    "Runtime Error: Failed to cast the {} provider field",
                    &field.get_ident().to_string()
                )
                .as_str(),
            );

            let token = field_instance.get_token();
            let module_exports = if field_instance.should_be_exported() {
                quote::quote! {
                    module.as_mut().register_exported_provider(token.clone());
                }
            } else {
                quote::quote! {}
            };

            compiler_providers.push(quote::quote! {
                module.as_mut().register_provider(#path_to_lib::cell::RefMut::new(
                  #path_to_lib::instance_wrapper::InstanceWrapper::new(
                    #token,
                      module.clone()
                  )
                ));

                #module_exports

                #provider_path::__compile(#token, module.clone(), container.clone());
            });

            init_providers.push(quote::quote! {
                #provider_path::__init(#token, module.clone(), container.clone());
            });
        }
    });

    let register_module = if module.get_attrs().is_module_global() {
        quote::quote! { container.as_mut().register_global_module(token.clone(), module.clone()); }
    } else {
        quote::quote! { container.as_mut().register_module(token.clone(), module.clone()); }
    };

    quote::quote! {
      impl #module_generics #path_to_lib::compiler::ModuleCompiler for #module_generics #module_ident {
        fn __compile(
          container: #path_to_lib::cell::RefMut<#path_to_lib::container::Container>,
          context: #path_to_lib::cell::RefMut<#path_to_lib::compiler::CompilerContext>
        ) -> #path_to_lib::cell::RefMut<#path_to_lib::module::Module> {
          let token = #path_to_lib::tokens::get_token::<#module_ident>();

          if context.as_ref().stack.contains(&token) {
              return container.as_ref().get_module(&token).unwrap();
          }

          let module = #path_to_lib::cell::RefMut::new(#path_to_lib::module::Module::new(token.clone()));
          module.as_mut().set_distance(context.as_ref().current_depth.clone());

          context.as_mut().stack.insert(token.clone());

          #register_module

          use #path_to_lib::compiler::ModuleCompiler; // for Module::__compile(...);
          use #path_to_lib::compiler::ProviderCompiler; // for Provider::__compile(...);

          // #(#compiler_providers)*
          #(#compiler_related_modules)*

          return module;
        }

        fn __init(
          container: #path_to_lib::cell::RefMut<#path_to_lib::container::Container>,
          context: #path_to_lib::cell::RefMut<#path_to_lib::compiler::CompilerContext>
        ) {
          let token = #path_to_lib::tokens::get_token::<#module_ident>();

          if context.as_ref().stack.contains(&token) {
              return;
          }

          context.as_mut().stack.insert(token.clone());

          let module = container.as_ref().get_module(&token).unwrap();

          use #path_to_lib::compiler::ModuleCompiler; // for Module::__compile(...);
          use #path_to_lib::compiler::ProviderCompiler; // for Provider::__compile(...);

          // #(#init_providers)*
          #(#init_related_modules)*

          // context.as_ref().finished(module);
        }
      }
    }
}
