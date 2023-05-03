use crate::config::Config;

pub fn logger_init(config: &Config) {
    tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_thread_ids(true)
        .with_target(false)
        .init();
}