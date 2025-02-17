use chrono::{DateTime, Utc};

pub mod basic;

pub trait Task: Send + Sync {
    fn get_next_execution(&self) -> Option<DateTime<Utc>>;
    fn call(&self);
}
