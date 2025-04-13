use std::{error::Error, pin::Pin, str::FromStr};

use crate::scheduler::{task_func::TaskFunc, tasks::CronTask};

use chrono::{DateTime, Utc};
use cron::Schedule;

pub struct BasicTask {
    schedule: Schedule,
    func: Box<dyn TaskFunc>,
}

impl BasicTask {
    pub fn new<F>(func: F, cron_expr: &str) -> Result<Self, Box<dyn Error>>
    where
        F: TaskFunc,
    {
        Ok(BasicTask {
            schedule: Schedule::from_str(cron_expr)?,
            func: Box::new(func),
        })
    }

    pub fn to_task(self) -> Box<dyn CronTask> {
        Box::new(self)
    }
}

impl CronTask for BasicTask {
    fn get_next_execution(&self) -> Option<DateTime<Utc>> {
        self.schedule.upcoming(Utc).next()
    }

    fn call(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async { (self.func)() })
    }
}
