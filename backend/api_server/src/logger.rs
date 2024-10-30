use std::path::Path;
use std::{env, io};

use chrono::Local;
use dirs::home_dir;
use tracing::info;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{
    filter::LevelFilter, fmt, fmt::format::Writer, fmt::time::FormatTime, fmt::MakeWriter,
    layer::SubscriberExt, Layer, Registry,
};

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(
            w,
            "{}",
            Local::now().format(constants::time::DEFAULT_TIME_FORMAT)
        )
    }
}

type _LogWriter = dyn for<'writer> MakeWriter<'writer, Writer = dyn io::Write + 'static> + 'static;

pub fn init() -> (WorkerGuard, WorkerGuard) {
    let log_path = &env::var("LOG_ROOT_DIR").unwrap_or_else(|e| {
        if let Ok(default_log_path) = env::var("WORK_DIR") {
            if default_log_path.starts_with('~') {
                let home_path = home_dir()
                    .expect("[fatal] unable to locate home dir with parsing env `WORK_DIR`, try to use absolute path instead");
                home_path
                    .join(default_log_path.strip_prefix("~/").unwrap())
                    .into_os_string()
                    .into_string()
                    .expect("[fatal] failed to parse env `WORK_DIR`, try to use absolute path instead")
            } else {
                default_log_path
            }
        } else {
            panic!("[fatal] failed to parse env `LOG_ROOT_DIR`: {}", e);
        }
    });

    let log_path = Path::new(log_path)
        .join("log")
        .join("backend")
        .into_os_string()
        .into_string()
        .unwrap_or_else(|e| panic!("[fatal] failed to parse {:?}", e));

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

    // let default_filter = tracing_subscriber::filter::EnvFilter::try_from_default_env()
    //     .unwrap_or_else(|_| "tower_http=debug,axum::rejection=trace".into());

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

    info!("module started: logger");
    info!("logging to dir: {}", log_path);

    (info_guard, error_guard)
}
