use std::path::Path;
use std::{env, io};

use crate::constants::config::general::{default_value, var_name};

use chrono::Local;
use dirs::home_dir;
use tracing::info;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{
    Layer, Registry, filter::LevelFilter, fmt, fmt::MakeWriter, fmt::format::Writer,
    fmt::time::FormatTime, layer::SubscriberExt,
};

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(
            w,
            "{}",
            Local::now().format(crate::constants::time::DEFAULT_TIME_FORMAT)
        )
    }
}

type _LogWriter = dyn for<'writer> MakeWriter<'writer, Writer = dyn io::Write + 'static> + 'static;

pub fn init() -> (WorkerGuard, WorkerGuard) {
    let log_path = match env::var(var_name::LOG_DIR) {
        Ok(path) => Path::new(&path).to_path_buf(),
        Err(_) => {
            let path = if let Ok(default_log_path) = env::var(var_name::WORK_DIR) {
                expand_home_path(default_log_path)
            } else {
                let fallback_path = expand_home_path(default_value::WORK_DIR.to_string());
                println!(
                    "[warn] unset env var `{}` and `{}`, trying to log on fallback path: {}/log",
                    var_name::LOG_DIR,
                    var_name::WORK_DIR,
                    fallback_path
                );
                fallback_path
            };
            Path::new(&path).join("log")
        }
    };

    let (info_log_writer, info_guard) = non_blocking(rolling::never(&log_path, "info.log"));
    let (err_log_writer, error_guard) = non_blocking(rolling::never(&log_path, "error.log"));

    let info_format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_source_location(true)
        .with_ansi(false);
    let err_format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_source_location(true)
        .with_ansi(false);
    let stdout_format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_source_location(true)
        .with_timer(LocalTimer);

    let subscribers = Registry::default()
        .with(
            fmt::Layer::default()
                .with_writer(info_log_writer)
                .event_format(info_format),
        )
        .with(
            fmt::Layer::default()
                .with_writer(err_log_writer)
                .event_format(err_format)
                .with_filter(LevelFilter::ERROR),
        )
        .with(
            fmt::Layer::default()
                .with_writer(io::stdout)
                .with_ansi(true)
                .event_format(stdout_format),
        );

    tracing::subscriber::set_global_default(subscribers)
        .expect("[fatal] failed to setup logging system");

    info!("logging to dir: {}", log_path.to_str().unwrap());

    (info_guard, error_guard)
}

fn expand_home_path(path: String) -> String {
    if path.starts_with('~') {
        let home_path = home_dir()
            .unwrap_or_else(|| panic!("[fatal] unable to locate home dir when parsing env `{}`, try to use absolute path instead", var_name::WORK_DIR));
        home_path
            .join(path.strip_prefix("~/").unwrap())
            .into_os_string()
            .into_string()
            .unwrap()
    } else {
        path
    }
}
