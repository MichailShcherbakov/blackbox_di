mod helpers;
mod modules;

use modules::gen_module_compiler;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let module = modules::parse_module(item).unwrap();

    let mut result = module.to_token_stream();

    result.extend(gen_module_compiler(&module));

    result.into()
}
