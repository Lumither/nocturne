use crate::blog;
use crate::scheduler::Scheduler;

pub fn start() {
    let scheduler = Scheduler::new();
    let _ = vec![blog::cron::get_tasks()]
        .into_iter()
        .map(|list| scheduler.insert_list(list))
        .collect::<Vec<_>>();
    scheduler.start().unwrap()
}
