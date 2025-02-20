mod constants;
mod init;
mod modules;
mod scheduler;
mod utils;

use crate::init::{logger, start};

const BUILD_ID: &str = env!("BUILD_ID");

#[tokio::main]
async fn main() {
    println!("{}", BUILD_ID);

    init::load_env();
    let _guards = logger::init();

    start().await;
}
