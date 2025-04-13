use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    sync::PoisonError,
};

use crate::scheduler::error::SchedulerError::MutexPoison;

// todo: refactor with thiserror
#[allow(dead_code)]
pub enum SchedulerError {
    IdNotFound,
    IdAlreadyRunning,
    MutexPoison(String),
}

impl<T> From<PoisonError<T>> for SchedulerError {
    fn from(value: PoisonError<T>) -> Self {
        MutexPoison(value.to_string())
    }
}

impl Debug for SchedulerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for SchedulerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SchedulerError::IdNotFound => {
                    "SchedulerError::IdNotFound".to_string()
                }
                SchedulerError::IdAlreadyRunning => {
                    "SchedulerError::IdAlreadyRunning".to_string()
                }
                MutexPoison(str) => {
                    format!("SchedulerError::MutexPoison: {}", str)
                }
            }
        )
    }
}

impl Error for SchedulerError {}
