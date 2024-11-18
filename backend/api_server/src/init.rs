use crate::logger;
use std::{
    backtrace::{Backtrace, BacktraceStatus},
    env, panic,
    panic::PanicHookInfo,
};
use tracing::{error, event, span, Level};

use tracing_appender::non_blocking::WorkerGuard;

pub fn logger_init() -> (WorkerGuard, WorkerGuard) {
    let ret = logger::init();
    panic::set_hook(Box::new(panic_hook));
    ret
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

fn panic_hook(panic_info: &PanicHookInfo) {
    let payload = panic_info.payload();
    // panic_info.

    let payload = if let Some(s) = payload.downcast_ref::<&str>() {
        Some(&**s)
    } else {
        payload.downcast_ref::<String>().map(|s| s.as_str())
    };

    // let location = panic_info.location().map(|l| l.to_string());
    //     let backtrace = Backtrace::capture();
    //     let note = (backtrace.status() == BacktraceStatus::Disabled)
    //         .then_some("run with RUST_BACKTRACE=1 environment variable to display a backtrace");
    //     (Some(backtrace), note)
    // };
    //
    // panic_info.

    if let Some(location) = panic_info.location() {
        eprintln!("err filename: {}", location.file())
    }

    event!(target: "test", Level::ERROR, "panic hook: {:?}", panic_info);
}
