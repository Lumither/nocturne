use crate::scheduler::task_func::TaskFunc;
use crate::scheduler::tasks::Task;
use chrono::{DateTime, Utc};
use cron::Schedule;
use std::error::Error;
use std::str::FromStr;

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

    pub fn to_task(self) -> Box<dyn Task> {
        Box::new(self)
    }
}

impl Task for BasicTask {
    fn get_next_execution(&self) -> Option<DateTime<Utc>> {
        self.schedule.upcoming(Utc).next()
    }

    fn call(&self) {
        (self.func)()
    }
}
