use syn::{parse::Result, Attribute};

const GLOBAL_MODULE_IDENT: &str = "global";

pub(crate) fn is_module_global(attrs: &mut Vec<Attribute>) -> Result<bool> {
    let mut is_module_global = false;

    attrs.retain(|attr| {
        if !attr.path.is_ident(GLOBAL_MODULE_IDENT) {
            return true;
        }

        is_module_global = true;

        return false;
    });

    Ok(is_module_global)
}

pub(crate) fn compile_error<'a>(msg: &'a str) {
    panic!("CompilerError: {}", msg);
}
