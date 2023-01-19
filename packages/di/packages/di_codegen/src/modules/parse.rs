use std::any::Any;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse, parse::ParseStream, Error, Field, Generics, Ident, ItemStruct, LitStr, Path};

use crate::helpers::{get_lazy_path, get_path_from_type, get_path_to_lib, get_ref_path};

use super::helpers::{compile_error, is_module_global};

static IMPORT_IDENT: &'static str = "import";
static PROVIDER_IDENT: &'static str = "provider";
static EXPORT_IDENT: &'static str = "export";

pub(crate) struct ImportField {}
pub(crate) struct ProviderField {
    token: TokenStream2,
    should_be_exported: bool,
}

impl ProviderField {
    pub fn get_token(&self) -> &TokenStream2 {
        &self.token
    }

    pub fn should_be_exported(&self) -> bool {
        self.should_be_exported.clone()
    }
}

#[derive(Debug)]
pub(crate) struct ModuleField {
    ident: Ident,
    path: TokenStream2,
    instance: Option<Box<dyn Any>>,
}

impl ModuleField {
    pub fn new(ident: Ident, path: TokenStream2) -> Self {
        Self {
            ident,
            path,
            instance: None,
        }
    }

    pub fn get_ident(&self) -> &Ident {
        &self.ident
    }

    pub fn get_path(&self) -> &TokenStream2 {
        &self.path
    }

    pub fn is<T: 'static>(&self) -> bool {
        if let Some(field) = &self.instance {
            field.is::<T>()
        } else {
            false
        }
    }

    pub fn as_field<T: 'static>(&self) -> Option<&T> {
        if let Some(field) = &self.instance {
            field.downcast_ref::<T>()
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub(crate) struct ModuleAttributes {
    path_to_lib: TokenStream2,
    is_module_global: bool,
}

impl ModuleAttributes {
    pub fn new(path_to_lib: TokenStream2, is_module_global: bool) -> ModuleAttributes {
        ModuleAttributes {
            path_to_lib,
            is_module_global,
        }
    }

    pub fn get_path_to_lib(&self) -> &TokenStream2 {
        &self.path_to_lib
    }

    pub fn is_module_global(&self) -> bool {
        self.is_module_global.clone()
    }
}

#[derive(Debug)]
pub(crate) struct Module {
    fields: Vec<ModuleField>,
    attrs: ModuleAttributes,
    source: ItemStruct,
}

impl Module {
    pub fn get_ident(&self) -> &Ident {
        &self.source.ident
    }

    pub fn get_generics(&self) -> &Generics {
        &self.source.generics
    }

    pub fn get_fields(&self) -> &Vec<ModuleField> {
        &self.fields
    }

    pub fn get_attrs(&self) -> &ModuleAttributes {
        &self.attrs
    }

    pub fn to_token_stream(&self) -> TokenStream2 {
        self.source.to_token_stream()
    }
}

pub(crate) fn parse_module(input: TokenStream) -> Result<Module, Error> {
    let mut item = parse::<ItemStruct>(input)?;

    let attrs = ModuleAttributes::new(
        get_path_to_lib(&mut item.attrs)?,
        is_module_global(&mut item.attrs)?,
    );

    let fields = item
        .fields
        .iter_mut()
        .map(|field| parse_module_field(field, &attrs).unwrap())
        .collect::<Vec<_>>();

    let module = Module {
        attrs,
        fields,
        source: item,
    };

    Ok(module)
}

fn parse_module_field(field: &mut Field, attrs: &ModuleAttributes) -> Result<ModuleField, Error> {
    let field_ident = field.ident.clone().unwrap();
    let field_path = get_path_from_type(&field.ty).unwrap();

    let mut module_field = ModuleField::new(field_ident.clone(), get_lazy_path(&field_path)?);

    let path_to_lib = &attrs.path_to_lib;

    field.attrs.retain(|attr| {
        if attr.path.is_ident(IMPORT_IDENT) {
            if let Some(module_field) = &module_field.instance {
                if module_field.is::<ImportField>() {
                    compile_error("The #[import] must be specified only once");
                } else if module_field.is::<ProviderField>() {
                    compile_error(
                        "The #[import] is incompatible with the #[provider]/#[export] tag",
                    );
                }
            } else {
                module_field.instance = Some(Box::new(ImportField {}));
            }

            return false;
        } else if attr.path.is_ident(PROVIDER_IDENT) {
            if let Some(module_field) = &module_field.instance {
                if module_field.is::<ImportField>() {
                    compile_error("The #[provider] is incompatible with the #[import] tag");
                } else if module_field.is::<ProviderField>() {
                    compile_error("The #[provider] must be specified only once");
                }
            } else {
                let provider_path = get_ref_path(&field_path).unwrap();

                let mut token = quote::quote! {
                    #path_to_lib::tokens::get_token::<#provider_path>()
                };

                if let Ok(parsed_token) =
                    attr.parse_args_with(|input: ParseStream| input.parse::<LitStr>())
                {
                    let val = parsed_token.value();

                    token = quote::quote! {
                      String::from(#val)
                    };
                } else if let Ok(token_path) =
                    attr.parse_args_with(|input: ParseStream| input.call(Path::parse_mod_style))
                {
                    token = quote::quote! {
                        #token_path.to_string()
                    };
                }

                module_field.instance = Some(Box::new(ProviderField {
                    token,
                    should_be_exported: false,
                }));
            }
            return false;
        } else if attr.path.is_ident(EXPORT_IDENT) {
            if let Some(module_field) = &mut module_field.instance {
                if module_field.is::<ImportField>() {
                    compile_error("The #[export] is incompatible with the #[import] tag");
                } else if let Some(module_field) = module_field.downcast_mut::<ProviderField>() {
                    module_field.should_be_exported = true;
                } else {
                    compile_error("The #[export] must be specified with the #[provider] tag");
                }
            } else {
                compile_error("The #[provider] tag is not specified or is below the #[export] tag");
            }

            return false;
        }

        return true;
    });

    Ok(module_field)
}
