use std::env;
use std::num::ParseIntError;
use std::process::exit;
use std::str::FromStr;

use crate::blog;
use axum::Router;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};
use tracing::{error, info, warn};

pub async fn start() {
    let port: u32 = match env::var("BACKEND_PORT") {
        Ok(value) => value.parse().unwrap_or_else(|e: ParseIntError| {
            warn!("failed to parse BACKEND_PORT: {}", e.to_string());
            3001
        }),
        Err(_) => 3001,
    };

    let db_connect_option = PgConnectOptions::from_str(&parse_db_uri())
        .unwrap()
        .disable_statement_logging();

    let db_pool = match PgPoolOptions::new().connect_with(db_connect_option).await {
        Ok(pool) => {
            info!("database connected");
            pool
        }
        Err(e) => {
            error!("failed to load database: {}", e.to_string());
            exit(1);
        }
    };

    let app = Router::new()
        .nest("blog", blog::get_router())
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap_or_else(|e| {
            error!("failed to build TCP listener: {}", e.to_string());
            exit(1);
        });

    axum::serve(listener, app).await.unwrap_or_else(|e| {
        error!("failed to start axum: {}", e.to_string());
        exit(1);
    });
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
