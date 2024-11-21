use crate::blog;
use crate::scheduler::Scheduler;
use sqlx::{Pool, Postgres};

pub fn start(db_pool: Pool<Postgres>) {
    let scheduler = Scheduler::new();
    let _ = vec![blog::cron::get_tasks(db_pool.clone())]
        .into_iter()
        .map(|list| scheduler.insert_list(list))
        .collect::<Vec<_>>();
    scheduler.start().unwrap()
}
