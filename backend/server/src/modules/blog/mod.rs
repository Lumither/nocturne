mod api;
mod cron;

use crate::{
    modules::{
        blog::{
            api::{
                get_page_count::get_page_count, get_post::get_post, get_post_list::get_post_list,
            },
            cron::check_update,
        },
        Module,
    },
    scheduler::tasks::basic::BasicTask,
    scheduler::tasks::CronTask,
};

use axum::{routing::get, Router};
use sqlx::PgPool;

const MOUNT_POINT: &str = "/blog";

pub struct Blog {
    pub db_handler: PgPool,
}

impl Module for Blog {
    fn get_server_router(&self) -> Router {
        let db_handler = self.db_handler.clone();
        // todo: remove new in get func?
        Router::new()
            .route("/get_post_list", get(get_post_list))
            .route("/get_page_count", get(get_page_count))
            .route("/get_post/:post_id", get(get_post))
            .with_state(db_handler)
    }

    fn get_mount_point(&self) -> &'static str {
        MOUNT_POINT
    }

    fn get_cron_tasks(&self) -> Vec<Box<dyn CronTask>> {
        let db_handler = self.db_handler.clone();
        vec![
            BasicTask::new(move || check_update::task(&db_handler), "0/15 * * * * *")
                .unwrap()
                .to_task(),
        ]
    }
}
