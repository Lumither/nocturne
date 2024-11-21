use crate::scheduler::tasks::basic::BasicTask;
use crate::scheduler::tasks::Task;

pub mod check_update;

pub fn get_tasks() -> Vec<Box<dyn Task>> {
    vec![BasicTask::new(check_update::task, "* * * * * *")
        .unwrap()
        .to_task()]
}
