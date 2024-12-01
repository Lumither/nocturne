mod cron;
mod database;
pub mod logger;
mod misc;
mod schema;
pub mod server;

use macros::panic_with_log;
use std::{env, panic};
use tracing::{info, Level};

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
            panic_with_log!(
                Level::ERROR,
                "database connection pool failed to initialize: {}",
                e
            );
        }
    };

    // schema init
    schema::init(&db_pool).await;

    // load cron task
    cron::start(db_pool.clone());

    // api serve
    match server::start(db_pool).await {
        Ok(_) => {}
        Err(e) => {
            panic_with_log!(Level::ERROR, "server panicked: {}", e)
        }
    }
}
