use crate::config::Config;
use anyhow::Result;
use tracing::level_filters::LevelFilter;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry,
};

pub fn logger_init(config: &Config) -> Result<WorkerGuard> {
    let log_path = config.log_path.as_ref().expect("Can not read log path");

    let formatting_layer = fmt::layer()
        // .pretty()
        .with_thread_ids(true)
        .with_target(false)
        .with_writer(std::io::stdout);

    let file_appender = rolling::daily(log_path, "rus-list.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .compact()
        .with_target(false)
        .with_thread_ids(true)
        .with_ansi(false)
        .with_writer(non_blocking);

    // let filter = filter::LevelFilter::INFO;
    let env_layer = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("RUA_LIST_LOG")
        .from_env_lossy();

    Registry::default()
        .with(env_layer)
        .with(ErrorLayer::default())
        .with(formatting_layer)
        .with(file_layer)
        .init();

    color_eyre::install().expect("");

    Ok(guard)
}
