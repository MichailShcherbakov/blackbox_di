use std::{
    any::Any,
    ops::Deref,
    sync::{Arc, Mutex},
};

use cast::{Cast, CastFrom, Error};

pub(crate) enum RefValue<T> {
    Initialized(T),
    WaitingForValue,
}

pub struct Ref<T: ?Sized + CastFrom> {
    value: Mutex<RefValue<Arc<T>>>,
}

unsafe impl<T: ?Sized + CastFrom> Sync for Ref<T> {}
unsafe impl<T: ?Sized + CastFrom> Send for Ref<T> {}

impl<T: CastFrom> Ref<T> {
    pub fn new(value: T) -> Ref<T> {
        Ref {
            value: Mutex::new(RefValue::Initialized(Arc::new(value))),
        }
    }

    pub fn empty() -> Ref<T> {
        Ref {
            value: Mutex::new(RefValue::WaitingForValue),
        }
    }
}

impl<T: ?Sized + CastFrom> Ref<T> {
    pub fn as_ref(&self) -> Arc<T> {
        if let RefValue::Initialized(value) = &*self.value.lock().unwrap() {
            value.clone()
        } else {
            panic!("Ref: Ref value must be initialized before the first usage")
        }
    }
}

impl<T: ?Sized + CastFrom> Ref<T> {
    pub fn init<D: ?Sized + CastFrom>(&self, value: Ref<D>) {
        *self.value.lock().unwrap() = RefValue::Initialized(value.cast::<T>().unwrap().as_ref());
    }
}

impl<T: ?Sized + CastFrom> Deref for Ref<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            if let RefValue::Initialized(value) =
                &*(self.value.lock().unwrap().deref() as *const RefValue<Arc<T>>)
            {
                value
            } else {
                panic!("Ref: Ref value must be initialized before the first usage")
            }
        }
    }
}

impl<T: ?Sized + CastFrom> Clone for Ref<T> {
    fn clone(&self) -> Ref<T> {
        Ref {
            value: Mutex::new(RefValue::Initialized(self.as_ref())),
        }
    }
}

impl<T: ?Sized + CastFrom> Ref<T> {
    pub fn cast<S: ?Sized + CastFrom>(&self) -> Result<Ref<S>, Error> {
        match self.as_ref().clone().cast::<S>() {
            Ok(value) => Ok(Ref {
                value: Mutex::new(RefValue::Initialized(value)),
            }),
            Err(error) => Err(error),
        }
    }

    pub fn is<S: ?Sized + Any + Send + Send>(&self) -> bool {
        self.as_ref().clone().is::<S>()
    }
}
