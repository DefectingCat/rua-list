use std::{process::exit, sync::Arc};

use anyhow::Result;
use log::{error, info};
use tokio::sync::Mutex;

use crate::config::Config;

mod arg;
mod config;
mod error;
mod logger;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(Mutex::new(Config::build()));

    let config = config.clone();
    if let Err(err) = logger::init_logger(config).await {
        error!("Failed to create logger; {}", err.to_string());
        exit(1);
    }

    info!("Server starting");

    Ok(())
}