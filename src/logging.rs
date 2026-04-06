use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling;
use tracing_subscriber::prelude::*;
use tracing_appender::non_blocking::WorkerGuard;

pub fn init() -> WorkerGuard {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // 👇 コンソールもJST
    let stdout_layer = fmt::layer()
        .with_timer(fmt::time::ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()));

    // 👇 ファイルもJST
    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_timer(fmt::time::ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()));

    tracing_subscriber::registry()
        .with(filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    guard
}