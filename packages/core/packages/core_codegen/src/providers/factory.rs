use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;

use super::{
    dependency_collection::ProviderDependencyCollection, helpers::gen_dep_ident,
    inject::gen_inject_deps_code, parse::Provider,
};

pub(crate) fn gen_inject_deps_factory_by_names(
    dep_collection: &ProviderDependencyCollection,
) -> TokenStream2 {
    let field_idents = dep_collection
        .iter()
        .map(|dependency| dependency.ident.clone());

    let dependency_names: Vec<Ident> = (0..dep_collection.len())
        .map(|idx| gen_dep_ident(idx))
        .collect();

    quote::quote! {
        {
            #(#field_idents: #dependency_names),*
        }
    }
}

pub(crate) fn gen_inject_deps_factory_by_tuple(
    dep_collection: &ProviderDependencyCollection,
) -> TokenStream2 {
    let dependencies: Vec<Ident> = (0..dep_collection.len())
        .map(|idx| gen_dep_ident(idx))
        .collect();

    quote::quote! {
        (#(#dependencies),*)
    }
}

pub(crate) fn gen_factory_code(provider: &Provider) -> TokenStream2 {
    let provider_ident = &provider.ident;

    let inject_deps_code = gen_inject_deps_code(&provider.deps, &provider.attrs);

    let (factory_ident, factory_code) = if provider.has_factory() {
        (
            provider.factory_ident.clone().unwrap(),
            gen_inject_deps_factory_by_tuple(&provider.deps),
        )
    } else {
        (
            provider.ident.clone(),
            gen_inject_deps_factory_by_names(&provider.deps),
        )
    };

    let path_to_lib = &provider.attrs.path_to_lib;

    quote::quote! {
        impl #path_to_lib::factory::Factory for #provider_ident {
            fn __blackbox_create() -> #path_to_lib::cell::Ref<#provider_ident> {
                #inject_deps_code

                #path_to_lib::cell::Ref::new(#factory_ident #factory_code)
            }
        }
    }
}
