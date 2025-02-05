use crate::scheduler::tasks::basic::BasicTask;
use crate::scheduler::tasks::Task;
use sqlx::{Pool, Postgres};

pub mod check_update;

pub fn get_tasks(db_connection: Pool<Postgres>) -> Vec<Box<dyn Task>> {
    let db_conn_clone = db_connection.clone();
    vec![
        // BasicTask::new(move || check_update::task(&db_conn_clone), "0/15 * * * * *")
        BasicTask::new(move || check_update::task(&db_conn_clone), "* * * * * *")
            .unwrap()
            .to_task(),
    ]
}
