pub trait TaskFunc: Fn() + Send + Sync + 'static {}

impl<T> TaskFunc for T where T: Fn() + Send + Sync + Clone + 'static {}
