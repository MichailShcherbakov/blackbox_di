use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::{
    parse, parse::ParseStream, spanned::Spanned, Error, FnArg, Generics, ImplItem, ItemFn,
    ItemImpl, ItemStruct, LitStr, Path, Token,
};

use crate::helpers::{get_path_from_type, get_path_to_lib, get_ref_path};

use super::{
    dependency::{Inject, ProviderDependency},
    dependency_collection::ProviderDependencyCollection,
    helpers::{gen_dep_ident, get_provider_ident_from_impl_block},
    scope::{get_scope, Scope},
};

const INJECT_IDENT: &str = "inject";
const FACTORY_IDENT: &str = "factory";

#[derive(Debug)]
pub struct ProviderAttributes {
    pub path_to_lib: TokenStream2,
    pub scope: Scope,
}

impl ProviderAttributes {
    fn new(path_to_lib: TokenStream2, scope: Scope) -> ProviderAttributes {
        ProviderAttributes { path_to_lib, scope }
    }
}

#[derive(Debug)]
pub struct Provider {
    pub ident: TokenStream2,
    pub generics: Option<Generics>,
    pub deps: ProviderDependencyCollection,
    pub factory_ident: Option<TokenStream2>,
    pub interface: Option<Path>,
    pub attrs: ProviderAttributes,
    source: TokenStream2,
}

impl Provider {
    pub fn to_token_stream(&self) -> TokenStream2 {
        self.source.clone()
    }

    pub fn has_interface(&self) -> bool {
        self.interface.is_some()
    }

    pub fn has_factory(&self) -> bool {
        self.factory_ident.is_some()
    }

    pub fn needs_factory(&self) -> bool {
        self.deps.has_uninjectable_deps()
    }
}

pub(crate) fn parse_provider(input: TokenStream) -> Result<Provider, Error> {
    if let Ok(item) = parse::<ItemStruct>(input.clone()) {
        let mut item = item;

        let attrs = ProviderAttributes::new(
            get_path_to_lib(&mut item.attrs)?,
            get_scope(&mut item.attrs)?,
        );

        let deps = parse_provider_deps_by_struct(&mut item, &attrs);

        let provider = Provider {
            ident: item.ident.clone().to_token_stream(),
            generics: Some(item.generics.clone()),
            deps,
            attrs,
            factory_ident: None,
            interface: None,
            source: item.to_token_stream(),
        };

        Ok(provider)
    } else if let Ok(item) = parse::<ItemImpl>(input.clone()) {
        let mut item = item;
        let provider_ident = get_provider_ident_from_impl_block(&item);

        let attrs = ProviderAttributes::new(
            get_path_to_lib(&mut item.attrs)?,
            get_scope(&mut item.attrs)?,
        );

        let factory_fn = detect_factory_method(&mut item);
        let mut factory_ident: Option<TokenStream2> = None;

        let deps = if let Ok(factory_fn) = factory_fn {
            let fn_ident = factory_fn.sig.ident.to_token_stream();
            let fn_prefix = quote::quote! { #provider_ident :: };

            factory_ident = Some(quote::quote! { #fn_prefix #fn_ident });

            parse_provider_deps_by_fn(&factory_fn)
        } else {
            ProviderDependencyCollection::new()
        };

        let interface: Option<Path> = if let Some((_, interface, _)) = &item.trait_ {
            Some(interface.clone())
        } else {
            None
        };

        let provider = Provider {
            ident: provider_ident,
            generics: None,
            deps,
            attrs,
            factory_ident,
            interface,
            source: item.to_token_stream(),
        };

        Ok(provider)
    } else {
        Err(Error::new(Span::call_site(), "Failed to parse provider"))
    }
}

fn parse_provider_deps_by_struct(
    item: &mut ItemStruct,
    attrs: &ProviderAttributes,
) -> ProviderDependencyCollection {
    let path_to_lib = &attrs.path_to_lib;

    item.fields
        .iter_mut()
        .map(|field| {
            let dep_ident = &field.ident.clone().unwrap();
            let dep_path = get_path_from_type(&field.ty).unwrap();

            let mut provider_dep =
                ProviderDependency::new(dep_ident.clone(), dep_path.clone(), None);

            field.attrs.retain(|attr| {
                if !attr.path.is_ident(INJECT_IDENT) {
                    return true;
                }

                let provider_path = get_ref_path(&dep_path, &attrs.path_to_lib).unwrap();

                let mut inject = Inject::new(quote::quote! {
                    #path_to_lib::tokens::get_token::<#provider_path>()
                });

                // #[inject("Token")]
                if let Ok(token) =
                    attr.parse_args_with(|input: ParseStream| input.parse::<LitStr>())
                {
                    let val = token.value();

                    inject.token = quote::quote! {
                      String::from(#val)
                    };
                // #[inject(STATIC_TOKEN)]
                } else if let Ok(token_path) =
                    attr.parse_args_with(|input: ParseStream| input.call(Path::parse_mod_style))
                {
                    inject.token = quote::quote! {
                      #token_path.to_string()
                    };
                // #[inject(use Provider)]
                } else if let Ok(provider_path) = attr.parse_args_with(|input: ParseStream| {
                    input.parse::<Token![use]>()?;
                    input.call(Path::parse_mod_style)
                }) {
                    inject.token = quote::quote! {
                        #path_to_lib::tokens::get_token::<#provider_path>()
                    }
                }

                provider_dep.inject = Some(inject);

                return false;
            });

            provider_dep
        })
        .collect::<ProviderDependencyCollection>()
}

fn parse_provider_deps_by_fn(item: &ItemFn) -> ProviderDependencyCollection {
    item.sig
        .inputs
        .iter()
        .enumerate()
        .map(|(idx, arg)| {
            let typed = if let FnArg::Typed(typed) = &arg {
                typed
            } else {
                panic!("Unsupported argument type")
            };

            let dep_ident = gen_dep_ident(idx);
            let dep_path = get_path_from_type(&typed.ty).unwrap();

            let provider_field = ProviderDependency::new(dep_ident.clone(), dep_path.clone(), None);

            provider_field
        })
        .collect::<ProviderDependencyCollection>()
}

fn detect_factory_method(impl_block: &mut ItemImpl) -> Result<ItemFn, Error> {
    let factory_fn = impl_block.items.iter_mut().find_map(|item| {
        if let ImplItem::Method(method) = item {
            let mut is_factory_attr_specified: bool = false;

            method.attrs.retain(|attr| {
                if !attr.path.is_ident(FACTORY_IDENT) {
                    return true;
                }

                is_factory_attr_specified = true;

                return false;
            });

            if is_factory_attr_specified {
                let mut fn_tokens = method.sig.to_token_stream();
                fn_tokens.extend(method.block.to_token_stream());

                let factory_fn = syn::parse::<ItemFn>(fn_tokens.into()).unwrap();

                return Some(factory_fn);
            } else {
                return None;
            }
        }

        return None;
    });

    if let Some(factory_fn) = factory_fn {
        Ok(factory_fn)
    } else {
        Err(Error::new(
            impl_block.span(),
            "Constructor with #[factory] attribute is not found",
        ))
    }
}
