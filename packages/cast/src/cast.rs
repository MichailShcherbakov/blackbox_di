use std::{any::Any, sync::Arc};

use crate::{cast_from::CastFrom, error::Error, vtable::get_trait_caster};

pub trait Cast {
    fn cast<T: ?Sized + Any>(self: Arc<Self>) -> Result<Arc<T>, Error>;
    fn is<T: ?Sized + Any>(self: Arc<Self>) -> bool;
}

impl<S: ?Sized + CastFrom> Cast for S {
    fn cast<T: ?Sized + Any>(self: Arc<Self>) -> Result<Arc<T>, Error> {
        match get_trait_caster::<T>((*self).type_id()) {
            Some(caster) => Ok((caster.cast_fn)(self.as_any())),
            None => Err(Error::new(
                format!(
                    "Not found {} trait caster for {}",
                    std::any::type_name::<T>(),
                    std::any::type_name::<S>()
                )
                .as_str(),
            )),
        }
    }

    fn is<T: ?Sized + 'static>(self: Arc<Self>) -> bool {
        self.cast::<T>().is_ok()
    }
}
