use std::{error::Error, pin::Pin, str::FromStr};

use crate::{scheduler::task_func::AsyncTaskFunc, scheduler::tasks::CronTask};

use chrono::{DateTime, Utc};
use cron::Schedule;

pub struct AsyncBasic {
    schedule: Schedule,
    func: Box<dyn AsyncTaskFunc>,
}

impl AsyncBasic {
    pub fn new<F>(func: F, cron_expr: &str) -> Result<Self, Box<dyn Error>>
    where
        F: AsyncTaskFunc,
    {
        Ok(AsyncBasic {
            schedule: Schedule::from_str(cron_expr)?,
            func: Box::new(func),
        })
    }

    pub fn to_task(self) -> Box<dyn CronTask> {
        Box::new(self)
    }
}

impl CronTask for AsyncBasic {
    fn get_next_execution(&self) -> Option<DateTime<Utc>> {
        self.schedule.upcoming(Utc).next()
    }

    fn call(&self) -> Pin<Box<(dyn Future<Output = ()> + Send)>> {
        Box::pin((self.func)())
    }
}
