use std::{
    any::Any,
    ops::Deref,
    sync::{Arc, Mutex},
};

use cast::{Cast, CastFrom, Error};

pub(crate) enum LinkValue<T> {
    Initialized(T),
    WaitingForValue,
}

pub struct Link<T: ?Sized + CastFrom> {
    value: Mutex<LinkValue<Arc<T>>>,
}

unsafe impl<T: ?Sized + CastFrom> Sync for Link<T> {}
unsafe impl<T: ?Sized + CastFrom> Send for Link<T> {}

impl<T: CastFrom> Link<T> {
    pub fn new(value: T) -> Link<T> {
        Link {
            value: Mutex::new(LinkValue::Initialized(Arc::new(value))),
        }
    }

    pub fn empty() -> Link<T> {
        Link {
            value: Mutex::new(LinkValue::WaitingForValue),
        }
    }
}

impl<T: ?Sized + CastFrom> Link<T> {
    pub fn as_ref(&self) -> Arc<T> {
        if let LinkValue::Initialized(value) = &*self.value.lock().unwrap() {
            value.clone()
        } else {
            panic!("Link: Link value must be initialized before the first usage")
        }
    }
}

impl<T: ?Sized + CastFrom> Link<T> {
    pub fn init<D: ?Sized + CastFrom>(&self, value: Link<D>) {
        *self.value.lock().unwrap() = LinkValue::Initialized(value.cast::<T>().unwrap().as_ref());
    }
}

impl<T: ?Sized + CastFrom> Deref for Link<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            if let LinkValue::Initialized(value) =
                &*(self.value.lock().unwrap().deref() as *const LinkValue<Arc<T>>)
            {
                value
            } else {
                panic!("Link: Link value must be initialized before the first usage")
            }
        }
    }
}

impl<T: ?Sized + CastFrom> Clone for Link<T> {
    fn clone(&self) -> Link<T> {
        Link {
            value: Mutex::new(LinkValue::Initialized(self.as_ref())),
        }
    }
}

impl<T: ?Sized + CastFrom> Link<T> {
    pub fn cast<S: ?Sized + CastFrom>(&self) -> Result<Link<S>, Error> {
        match self.as_ref().clone().cast::<S>() {
            Ok(value) => Ok(Link {
                value: Mutex::new(LinkValue::Initialized(value)),
            }),
            Err(error) => Err(error),
        }
    }

    pub fn is<S: ?Sized + Any + Send + Send>(&self) -> bool {
        self.as_ref().clone().is::<S>()
    }
}
