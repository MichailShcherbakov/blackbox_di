use std::collections::HashSet;

use cast::CastFrom;

use crate::{
    container::Container,
    instance_wrapper::InstanceToken,
    module::{Module, ModuleToken},
    reference::Ref,
    reference_mut::RefMut,
};

pub trait Injectable {}

pub trait ProviderCompiler {
    fn __compile(
        token: InstanceToken,
        module: RefMut<Module>,
        container: RefMut<Container>,
    ) -> Ref<Self>
    where
        Self: CastFrom;

    fn __init(token: InstanceToken, module: RefMut<Module>, container: RefMut<Container>);
}

pub struct CompilerContext {
    pub stack: HashSet<ModuleToken>,
    pub current_depth: u32,
}

pub fn new() -> CompilerContext {
    CompilerContext {
        stack: HashSet::new(),
        current_depth: 0,
    }
}

pub trait ModuleCompiler {
    fn __compile(container: RefMut<Container>, context: RefMut<CompilerContext>) -> RefMut<Module>;

    fn __init(container: RefMut<Container>, context: RefMut<CompilerContext>);
}
