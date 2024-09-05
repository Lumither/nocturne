mod init;
mod logger;

fn main() {
    init::load_env();
    let _guards = init::logger_init();
}
