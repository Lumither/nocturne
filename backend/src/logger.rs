use std::{env, io};

use chrono::Local;
use tracing::{error, info, Level};
use tracing_appender::{non_blocking, rolling};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, Registry};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;

use crate::constants::GLOBAL_TIME_FORMAT;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format(GLOBAL_TIME_FORMAT))
    }
}

type LogWriter = dyn for<'writer> MakeWriter<'writer, Writer = dyn io::Write + 'static> + 'static;

pub fn init() -> (WorkerGuard, WorkerGuard) {
    let log_path = env::var("LOG_ROOT_DIR").unwrap_or_else(|e| {
        panic!("[Fatal] failed to load LOG_ROOT_DIR: {}", e);
    });

    let log_path = format!("{}/backend", log_path);

    let (info_log_writer, info_guard) = non_blocking(rolling::never(&log_path, "info.log"));
    // let info_log = info_log.with_max_level(Level::TRACE);

    let (err_log_writer, error_guard) = non_blocking(rolling::never(&log_path, "error.log"));
    // let error_log = error_logging_handle.with_filter(|log| log.level().eq(&Level::ERROR));

    let log_format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_source_location(true)
        .with_timer(LocalTimer);

    // todo: separate errors/warnings/info logs
    let subscribers = Registry::default()
        .with(
            fmt::Layer::default()
                .with_writer(info_log_writer)
                // .with_max_level(Level::TRACE)
                .with_ansi(false),
        )
        .with(
            fmt::Layer::default()
                .with_writer(err_log_writer)
                .with_ansi(false),
        )
        .with(
            fmt::Layer::default()
                .with_writer(io::stdout)
                .with_ansi(true)
                .event_format(log_format),
        );

    tracing::subscriber::set_global_default(subscribers)
        .expect("[Fatal] Failed to setup logging system");

    info!("Module Started: Logger");

    // error!("test");
    (info_guard, error_guard)
}
