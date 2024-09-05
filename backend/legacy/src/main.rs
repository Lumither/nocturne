use std::env;
use std::error::Error;
use std::num::ParseIntError;
use std::process::exit;
use std::str::FromStr;

use crate::api::{
    get::{get_page_count::get_page_count, get_post::get_post, get_post_list::get_post_list},
    post::refresh::refresh,
};
use axum::routing::{get, post};
use axum::Router;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::ConnectOptions;
use tracing::{error, info, warn};

mod api;
mod constants;
mod logger;
mod markdown;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Some(env_file) = env::args().nth(1) {
        match dotenv::from_filename(&env_file) {
            Ok(_) => {
                println!("[Info] env loaded from {}, starting up", env_file);
            }
            Err(e) => {
                panic!("[Fatal] failed to read {}: {}", env_file, e);
            }
        }
    } else {
        println!("[info] no .env file referred, starting up")
    }

    let _guards = logger::init();

    let port: u32 = match env::var("BACKEND_PORT") {
        Ok(value) => value.parse().unwrap_or_else(|e: ParseIntError| {
            warn!("Failed to parse BACKEND_PORT: {}", e.to_string());
            3001
        }),
        Err(_) => 3001,
    };

    let db_connect_option = PgConnectOptions::from_str(&parse_db_uri())
        .unwrap()
        .disable_statement_logging();

    let db_pool = match PgPoolOptions::new().connect_with(db_connect_option).await {
        Ok(pool) => {
            info!("Database connected");
            pool
        }
        Err(e) => {
            error!("Failed to load database: {}", e.to_string());
            exit(1);
        }
    };

    let app = Router::new()
        .route("/refresh_posts", post(refresh))
        .route("/get_post_list", get(get_post_list))
        .route("/get_page_count", get(get_page_count))
        .route("/get_post/:post_id", get(get_post))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap_or_else(|e| {
            error!("Failed to build TCP listener: {}", e.to_string());
            exit(1);
        });

    axum::serve(listener, app).await.unwrap_or_else(|e| {
        error!("Failed to start axum: {}", e.to_string());
        exit(1);
    });

    Ok(())
}

fn parse_db_uri() -> String {
    let postgres_user = env::var("POSTGRES_USER").unwrap_or("".to_string());
    let postgres_password = env::var("POSTGRES_PASSWORD").unwrap_or("".to_string());
    let db_host = env::var("DB_HOST").unwrap_or("localhost".to_string());
    let db_port = env::var("DB_PORT")
        .unwrap_or("5432".to_string())
        .to_string();
    let postgres_db = env::var("POSTGRES_DB").unwrap_or("nocturne".to_string());

    format!("postgresql://{postgres_user}:{postgres_password}@{db_host}:{db_port}/{postgres_db}")
}
