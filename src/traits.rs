use std::any::Any;

/// The data in each POBJ must be clonable and sent through threads
/// must it really be sent through threads? I need to think about this
pub trait CloneableAny: Any + Send + Sync {
    fn clone_box(&self) -> Box<dyn CloneableAny>;
}

impl<T> CloneableAny for T
where
    T: Any + Send + Sync + Clone,
{
    fn clone_box(&self) -> Box<dyn CloneableAny> {
        Box::new(self.clone())
    }
}