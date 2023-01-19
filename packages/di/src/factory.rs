use cast::CastFrom;

use crate::{
    container::Container, instance_wrapper::InstanceToken, module::Module, reference_mut::RefMut,
    reference::Ref,
};

pub trait Factory<T: ?Sized + CastFrom> {
    fn create(token: InstanceToken, module: Ref<Module>, container: RefMut<Container>) -> Ref<T>;
}

pub struct DefaultFactory {}

impl DefaultFactory {
    pub fn new() -> DefaultFactory {
        DefaultFactory {}
    }
}
