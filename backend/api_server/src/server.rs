use std::env;
use std::error::Error;
use std::num::ParseIntError;
use std::process::exit;
use std::str::FromStr;

use crate::blog;
use axum::Router;
use macros::panic_with_log;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};
use tracing::{error, info, warn, Level};

pub async fn start() -> Result<(), Box<dyn Error>> {
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

    // cron task
    // let scheduler = Scheduler::new();
    // let test_str = "hello world".to_string();
    // scheduler.insert(
    //     BasicTask::new(
    //         move || blog::cron::check_update::task(test_str.clone()),
    //         "* * * * * *",
    //     )?
    //     .to_task(),
    // )?;

    // http server
    let app = Router::new()
        .nest("/blog", blog::get_router())
        .with_state(db_pool);

    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        Ok(listener) => {
            info!("server started on 0.0.0.0:{}", port);
            listener
        }
        Err(e) => {
            panic_with_log!(
                Level::ERROR,
                "failed to start server on 0.0.0.0:{}: {}",
                port,
                e.to_string()
            );
        }
    };

    axum::serve(listener, app).await.unwrap_or_else(|e| {
        error!("failed to start axum: {}", e.to_string());
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
