mod database;
pub mod logger;
mod misc;
mod schema;

use std::{env, panic};

use crate::modules::{blog::Blog, ModuleTree};
use macros::error_panic;

use tracing::info;

pub fn load_env() {
    if let Some(env_file) = env::args().nth(1) {
        match dotenv::from_filename(&env_file) {
            Ok(_) => {
                println!("[info] env loaded from {}, starting up", env_file);
            }
            Err(e) => {
                panic!("[fatal] failed to read {}: {}", env_file, e);
            }
        }
    } else {
        println!("[info] no .env file referred, starting up")
    }
}

pub async fn start() {
    misc::init();

    // connect database
    let db_pool = match database::init().await {
        Ok(pool) => {
            info!("database connection pool initialized");
            pool
        }
        Err(e) => {
            error_panic!("database connection pool failed to initialize: {}", e);
        }
    };

    // // schema init
    // schema::init(&db_pool).await;

    ModuleTree::new()
        .add_module(Blog {
            db_handler: db_pool,
        })
        .blocking_serve()
        .await
}
