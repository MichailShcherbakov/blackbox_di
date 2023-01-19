use std::{any::Any, sync::Arc};

pub trait CastFrom: Any + Send + Sync {
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T: Sized + Send + Sync + Any> CastFrom for T {
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self.clone()
    }
}
