mod blog;
mod init;
mod scheduler;

#[tokio::main]
async fn main() {
    init::load_env();

    let _guards = init::logger::init();

    init::start().await;
}
