/// log the message at specific log level and exit(1)
///
/// # Example
/// ```rust
///
/// use macros::panic_with_log;
/// use tracing::Level;
///
/// panic_with_log!(Level::ERROR, "Test panicked: {}", 42);
/// ```
#[macro_export]
macro_rules! panic_with_log {
    ($level:expr, $msg:literal $(, $($arg:tt)*)?) => {{
        use std::process;
        use tracing::event;
        event!($level, $msg $(, $($arg)*)?);
        process::exit(1);
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_panic_with_log() {
        use std::io;
        use tracing::Level;
        use tracing_subscriber::prelude::*;
        use tracing_subscriber::{fmt, Registry};

        let format = tracing_subscriber::fmt::format()
            .with_level(true)
            .with_target(true)
            .with_source_location(true);

        let subscriber = Registry::default().with(
            fmt::Layer::default()
                .with_writer(io::stderr)
                .event_format(format),
        );

        tracing::subscriber::set_global_default(subscriber).unwrap();

        panic_with_log!(Level::ERROR, "Test panicked: {}", 42);
    }
}
