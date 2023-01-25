use std::{
    any::Any,
    ops::Deref,
    sync::{Arc, Mutex},
};

use cast::{Cast, CastFrom, Error};

pub enum RefValue<T: ?Sized> {
    Initialized(Arc<T>),
    WaitingForValue,
}

pub struct Ref<T: ?Sized> {
    value: Arc<Mutex<RefValue<T>>>,
}

unsafe impl<T: ?Sized> Sync for Ref<T> {}
unsafe impl<T: ?Sized> Send for Ref<T> {}

impl<T: CastFrom> Ref<T> {
    pub fn new(value: T) -> Ref<T> {
        Ref {
            value: Arc::new(Mutex::new(RefValue::Initialized(Arc::new(value)))),
        }
    }
}

impl<T: ?Sized + CastFrom> Ref<T> {
    pub fn as_ref(&self) -> Arc<T> {
        if let RefValue::Initialized(value) = &*self.value.lock().unwrap() {
            value.clone()
        } else {
            panic!("Ref: value must be initialized before the first usage")
        }
    }

    pub fn empty() -> Ref<T> {
        Ref {
            value: Arc::new(Mutex::new(RefValue::WaitingForValue)),
        }
    }

    pub fn __init<D: ?Sized + CastFrom>(&self, value: Ref<D>) {
        *self.value.lock().unwrap() = RefValue::Initialized(value.cast::<T>().unwrap().as_ref());
    }

    pub fn cast<S: ?Sized + CastFrom>(&self) -> Result<Ref<S>, Error> {
        match self.as_ref().cast::<S>() {
            Ok(value) => Ok(Ref {
                value: Arc::new(Mutex::new(RefValue::Initialized(value))),
            }),
            Err(error) => Err(error),
        }
    }

    pub fn is<S: ?Sized + Any + Sync + Send>(&self) -> bool {
        self.as_ref().clone().is::<S>()
    }
}

impl<T: ?Sized + CastFrom> Deref for Ref<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            if let RefValue::Initialized(value) =
                &*(self.value.lock().unwrap().deref() as *const RefValue<T>)
            {
                value
            } else {
                panic!("Ref: value must be initialized before the first usage")
            }
        }
    }
}

impl<T: ?Sized + CastFrom> Clone for Ref<T> {
    fn clone(&self) -> Ref<T> {
        Ref {
            value: self.value.clone(),
        }
    }
}
