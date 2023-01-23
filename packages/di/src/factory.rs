use cast::CastFrom;

use crate::reference::Ref;

pub trait Factory {
    fn __blackbox_create() -> Ref<Self>
    where
        Self: CastFrom;
}
