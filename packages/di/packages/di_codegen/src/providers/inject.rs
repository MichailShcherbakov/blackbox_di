use proc_macro2::TokenStream as TokenStream2;

use crate::helpers::get_ref_path;

use super::{
    dependency::ProviderDependency,
    dependency_collection::ProviderDependencyCollection,
    helpers::gen_dep_ident,
    parse::{Provider, ProviderAttributes},
};

pub(crate) fn gen_inject_deps_code(
    dep_collection: &ProviderDependencyCollection,
    attrs: &ProviderAttributes,
) -> TokenStream2 {
    let inject_deps_code = dep_collection
        .iter()
        .enumerate()
        .map(|(idx, dep)| gen_inject_dep_code(idx, dep, attrs))
        .collect::<Vec<_>>();

    quote::quote! {
      #(#inject_deps_code)*
    }
}

pub(crate) fn gen_inject_dep_code(
    uident: usize,
    dep: &ProviderDependency,
    attrs: &ProviderAttributes,
) -> TokenStream2 {
    let ref_path = get_ref_path(&dep.path).unwrap();
    let ident = gen_dep_ident(uident);
    let path_to_lib = &attrs.path_to_lib;

    quote::quote! {
        let #ident = #path_to_lib::cell::Ref::<#ref_path>::empty();
    }
}

pub(crate) fn gen_register_deps(provider: &Provider) -> Vec<TokenStream2> {
    let provide_ident = &provider.ident;

    provider
        .deps
        .get_injectable_deps()
        .iter()
        .map(|dep| {
            let dep_ident = &dep.ident;
            let dep_path = get_ref_path(&dep.path).unwrap();

            let token = &dep
                .inject
                .as_ref()
                .expect("Inject token was not provided")
                .token;

            quote::quote! {
                register_dependency::<#provide_ident, #dep_path>(#token, |self_, dep| self_.#dep_ident.__init(dep))
            }
        })
        .collect::<Vec<_>>()
}
