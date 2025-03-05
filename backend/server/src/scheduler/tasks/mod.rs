use std::pin::Pin;

use chrono::{DateTime, Utc};

pub mod async_basic;
pub mod basic;

pub trait CronTask: Send + Sync {
    fn get_next_execution(&self) -> Option<DateTime<Utc>>;
    fn call(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}
