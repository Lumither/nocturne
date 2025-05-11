mod api;
mod components;

use crate::{
    modules::{
        Module,
        blog::{
            api::{posts, posts::pagination},
            components::{check_update, consistency_guard},
        },
    },
    scheduler::tasks::{CronTask, async_basic::AsyncBasic},
};

use axum::{Router, routing::get};
use sqlx::PgPool;

const MOUNT_POINT: &str = "/blog";

pub struct Blog {
    pub db_handler: PgPool,
}

impl Module for Blog {
    fn get_server_router(&self) -> Router {
        let db_handler = self.db_handler.clone();
        // todo: move resource init to new func
        // todo: remove debug comment
        Router::new()
            // .route("/get_post_list", get(get_post_list))
            // .route("/posts/pagination", get(...))
            // .route("/posts", get(...)) // get posts
            .route("/posts/pagination", get(pagination::handler))
            .route("/posts/{identifier}", get(posts::get::handler))
            .with_state(db_handler)
    }

    fn get_mount_point(&self) -> &'static str {
        MOUNT_POINT
    }

    fn get_cron_tasks(&self) -> Vec<(&str, Box<dyn CronTask>)> {
        let db_handler = self.db_handler.clone();
        vec![
            (
                "Blog Check Update",
                AsyncBasic::new(check_update::task(db_handler.clone()), "*/5 * * * * *")
                    .unwrap()
                    .to_task(),
            ),
            (
                "Consistency Guard",
                AsyncBasic::new(
                    consistency_guard::task(db_handler.clone()),
                    "*/15 * * * * *",
                )
                .unwrap()
                .to_task(),
            ),
        ]
    }
}
