use std::{any::Any, sync::Arc};

pub struct Caster<T: ?Sized + Any> {
    pub cast_fn: fn(from: Arc<dyn Any + Sync + Send>) -> Arc<T>,
}

impl<T: ?Sized + Any> Caster<T> {
    pub fn new(cast_fn: fn(from: Arc<dyn Any + Sync + Send>) -> Arc<T>) -> Self {
        Self { cast_fn }
    }
}
