use std::env;

use crate::logger;

use tracing_appender::non_blocking::WorkerGuard;

pub fn logger_init() -> (WorkerGuard, WorkerGuard) {
    logger::init()
}

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
