use crate::blog::scheduler::task_func::TaskFunc;
use crate::blog::scheduler::tasks::Task;
use chrono::{DateTime, Utc};
use cron::Schedule;
use std::error::Error;
use std::str::FromStr;

pub struct BasicTask {
    schedule: Schedule,
    func: Box<dyn TaskFunc>,
}

impl BasicTask {
    pub fn new(func: fn(), cron_expr: &str) -> Result<Self, Box<dyn Error>> {
        Ok(BasicTask {
            schedule: Schedule::from_str(cron_expr)?,
            func: Box::new(func),
        })
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
