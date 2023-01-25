use std::sync::{Arc, RwLock};

pub struct RefMut<T: ?Sized> {
    value: Arc<RwLock<T>>,
}

unsafe impl<T: ?Sized> Sync for RefMut<T> {}
unsafe impl<T: ?Sized> Send for RefMut<T> {}

impl<T> RefMut<T> {
    pub fn new(value: T) -> RefMut<T> {
        RefMut {
            value: Arc::new(RwLock::new(value)),
        }
    }
}

type RefMutRef<'a, T> = std::sync::RwLockReadGuard<'a, T>;
type RefMutRefMut<'a, T> = std::sync::RwLockWriteGuard<'a, T>;

impl<T: ?Sized> RefMut<T> {
    pub fn as_ref(&self) -> RefMutRef<T> {
        self.value.as_ref().read().unwrap()
    }

    pub fn as_mut(&self) -> RefMutRefMut<T> {
        self.value.as_ref().write().unwrap()
    }
}

impl<T: ?Sized> Clone for RefMut<T> {
    fn clone(&self) -> RefMut<T> {
        RefMut {
            value: self.value.clone(),
        }
    }
}
