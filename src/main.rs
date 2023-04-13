use std::process::exit;

use anyhow::Result;
use log::{error, info};

use crate::config::Config;

mod arg;
mod config;
mod error;
mod logger;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::build();
    if let Err(err) = logger::init_logger(&config).await {
        error!("Failed to create logger; {}", err.to_string());
        exit(1);
    }

    info!("Server starting");

    println!("{config:?}");

    Ok(())
}