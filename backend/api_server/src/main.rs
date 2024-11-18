mod blog;
mod init;
mod logger;
mod scheduler;
mod server;

#[tokio::main]
async fn main() {
    init::load_env();
    let _guards = init::logger_init();

    // panic!("test");

    match server::start().await {
        Ok(_) => {}
        Err(e) => {
            panic!("server start failed: {}", e);
        }
    };
}
