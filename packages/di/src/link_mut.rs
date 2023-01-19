use std::sync::{Arc, RwLock};

pub struct LinkMut<T: ?Sized> {
    value: Arc<RwLock<T>>,
}

unsafe impl<T: ?Sized> Sync for LinkMut<T> {}
unsafe impl<T: ?Sized> Send for LinkMut<T> {}

impl<T> LinkMut<T> {
    pub fn new(value: T) -> LinkMut<T> {
        LinkMut {
            value: Arc::new(RwLock::new(value)),
        }
    }
}

type LinkMutRef<'a, T> = std::sync::RwLockReadGuard<'a, T>;
type LinkMutRefMut<'a, T> = std::sync::RwLockWriteGuard<'a, T>;

impl<T: ?Sized> LinkMut<T> {
    pub fn as_ref(&self) -> LinkMutRef<T> {
        self.value.as_ref().read().unwrap()
    }

    pub fn as_mut(&self) -> LinkMutRefMut<T> {
        self.value.as_ref().write().unwrap()
    }
}

impl<T: ?Sized> Clone for LinkMut<T> {
    fn clone(&self) -> LinkMut<T> {
        LinkMut {
            value: self.value.clone(),
        }
    }
}
