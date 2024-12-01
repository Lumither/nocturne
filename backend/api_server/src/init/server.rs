use std::env;
use std::error::Error;
use std::num::ParseIntError;
use std::process::exit;

use crate::blog;
use axum::Router;
use constants::config::server::{default_value, var_name};
use macros::panic_with_log;
use sqlx::{Pool, Postgres};
use tracing::{error, info, warn, Level};

pub async fn start(db_pool: Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .nest("/blog", blog::get_router())
        .with_state(db_pool);

    let port: u32 = match env::var(var_name::PORT) {
        Ok(value) => value.parse().unwrap_or_else(|e: ParseIntError| {
            warn!(
                "failed to parse `{}`, using default port: {}",
                var_name::PORT,
                e.to_string()
            );
            default_value::PORT
        }),
        Err(_) => default_value::PORT,
    };

    let listener =
        match tokio::net::TcpListener::bind(format!("{}:{}", default_value::HOST, port)).await {
            Ok(listener) => {
                info!("server started on {}:{}", default_value::HOST, port);
                listener
            }
            Err(e) => {
                panic_with_log!(
                    Level::ERROR,
                    "failed to start server on {}:{}: {}",
                    default_value::HOST,
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
