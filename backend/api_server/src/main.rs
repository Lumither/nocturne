use macros::panic_with_log;
use tracing::Level;

mod blog;
mod init;
mod logger;
mod scheduler;
mod server;

#[tokio::main]
async fn main() {
    init::load_env();
    let _guards = init::logger_init();

    match server::start().await {
        Ok(_) => {}
        Err(e) => {
            panic_with_log!(Level::ERROR, "failed to start server: {}", e);
        }
    };
}
