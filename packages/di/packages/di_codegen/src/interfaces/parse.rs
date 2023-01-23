use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse, Error, ItemTrait};

use crate::helpers::get_path_to_lib;

#[derive(Debug)]
pub struct InterfaceAttributes {
    pub path_to_lib: TokenStream2,
}

#[derive(Debug)]
pub struct Interface {
    attrs: InterfaceAttributes,
    source: ItemTrait,
}

impl Interface {
    pub fn to_token_stream(&self) -> TokenStream2 {
        let path_to_lib = &self.attrs.path_to_lib;
        let ItemTrait {
            ref ident,
            ref generics,
            ref vis,
            ref items,
            ref supertraits,
            ref trait_token,
            ..
        } = self.source;

        quote::quote! {
            #[#path_to_lib::async_trait::async_trait]
            #vis #trait_token #ident #generics: #path_to_lib::cast::CastFrom + #supertraits {
                #(#items)*
            }
        }
    }
}

pub(crate) fn parse_interface(input: TokenStream) -> Result<Interface, Error> {
    let mut item = parse::<ItemTrait>(input.clone())?;

    let path_to_lib = get_path_to_lib(&mut item.attrs)?;

    let attrs = InterfaceAttributes { path_to_lib };

    let interface = Interface {
        source: item,
        attrs,
    };

    Ok(interface)
}
