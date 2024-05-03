use std::env;
use std::error::Error;

use axum::Router;
use axum::routing::{get, post};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::api::get::get_post::get_post;
use crate::api::get::get_post_list::get_post_list;
use crate::api::post::refresh::refresh;

mod api;
mod model;
mod markdown;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let port: u32 = match env::var("BACKEND_PORT") {
        Ok(value) => value.parse().unwrap(),
        Err(_) => 3001,
    };

    let db_connection: String = env::var("DB_CONNECTION")?;

    let db_pool = PgPoolOptions::new()
        .connect(&db_connection)
        .await
        .expect("failed to load database");

    let app = Router::new()
        .route("/api/post/refresh", post(refresh))
        .route("/api/get/post_list", get(get_post_list))
        .route("/api/get/post/:post_id", get(get_post))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
