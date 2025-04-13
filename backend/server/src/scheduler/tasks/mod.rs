use std::pin::Pin;

use chrono::{DateTime, Utc};

#[allow(dead_code)]
pub mod async_basic;

#[allow(dead_code)]
pub mod basic;

pub trait CronTask: Send + Sync {
    fn get_next_execution(&self) -> Option<DateTime<Utc>>;
    fn call(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}
