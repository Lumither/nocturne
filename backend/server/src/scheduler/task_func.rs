use std::pin::Pin;

pub trait TaskFunc: Fn() + Send + Sync + 'static {}

impl<T> TaskFunc for T where T: Fn() + Send + Sync + Clone + 'static {}

pub trait AsyncTaskFunc:
    Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static
{
}

impl<T> AsyncTaskFunc for T where
    T: Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + Clone + 'static
{
}
