use crate::config::Config;
use anyhow::Result;
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter, fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Registry,
};

pub fn logger_init(config: &Config) -> Result<()> {
    let log_path = config.log_path.as_ref().expect("Can not read log path");

    let formatting_layer = fmt::layer()
        // .pretty()
        .with_thread_ids(true)
        .with_target(false)
        .with_writer(std::io::stdout);

    let file_appender = rolling::daily(log_path, "rua-list.log");
    let (non_blocking, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .compact()
        .with_target(false)
        .with_thread_ids(true)
        .with_writer(non_blocking);

    let filter = filter::LevelFilter::INFO;

    Registry::default()
        .with(filter)
        .with(ErrorLayer::default())
        .with(formatting_layer)
        .with(file_layer)
        .init();

    Ok(())
}