mod blog;
mod init;
mod logger;
mod server;

#[tokio::main]
async fn main() {
    init::load_env();
    let _guards = init::logger_init();

    server::start().await;
}
