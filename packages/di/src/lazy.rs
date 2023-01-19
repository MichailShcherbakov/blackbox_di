use std::sync::Arc;

pub struct Lazy<T>(Arc<T>);
