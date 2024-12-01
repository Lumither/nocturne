mod api;
pub mod cron;

use crate::blog::api::{
    get_page_count::get_page_count, get_post::get_post, get_post_list::get_post_list,
};

use axum::routing::get;
use axum::Router;
use sqlx::{Pool, Postgres};

pub fn get_router() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/get_post_list", get(get_post_list))
        .route("/get_page_count", get(get_page_count))
        .route("/get_post/:post_id", get(get_post))
}
