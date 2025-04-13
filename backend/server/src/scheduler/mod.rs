use std::{collections::HashMap, sync::Arc};

use crate::scheduler::{error::SchedulerError, tasks::CronTask, utils::execute};

use futures::future::join_all;
use tracing::warn;

pub mod error;
pub mod task_func;
pub mod tasks;
mod utils;

pub type TaskPool = HashMap<String, Arc<Box<dyn CronTask>>>;

#[derive(Default)]
pub struct Scheduler {
    task_pool: TaskPool,
}

impl Scheduler {
    /// Start all tasks in the scheduler
    pub async fn start(&self) {
        let fut = self
            .task_pool
            .clone()
            .into_iter()
            .map(|(name, task)| execute(name, task))
            .collect::<Vec<_>>();
        // self.runner.run_all().await
        join_all(fut).await;
    }

    pub fn insert(&mut self, task: (&str, Box<dyn CronTask>)) -> Result<&Self, SchedulerError> {
        let (k, v) = task;
        if self.task_pool.insert(k.into(), v.into()).is_some() {
            warn!("task list duplicated key detected: {}", k);
        }
        Ok(self)
    }

    pub fn insert_list(
        &mut self,
        tasks: Vec<(&str, Box<dyn CronTask>)>,
    ) -> Result<&Self, SchedulerError> {
        for task in tasks {
            self.insert(task)?;
        }
        Ok(self)
    }

    pub fn new() -> Self {
        Self::default()
    }
}
