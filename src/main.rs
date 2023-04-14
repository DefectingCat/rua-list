use std::{net::SocketAddr, process::exit, sync::Arc};

use anyhow::Result;
use axum::{routing::get, Router, Server};
use log::{error, info};
use tokio::sync::Mutex;

use crate::{config::Config, routes::messages::get_sms_aspx};

mod arg;
mod config;
mod error;
mod logger;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(Mutex::new(Config::build()));

    let log_config = config.clone();
    if let Err(err) = logger::init_logger(log_config).await {
        error!("Failed to create logger; {}", err.to_string());
        exit(1);
    }

    let port = if let Some(port) = config.clone().lock().await.port.to_owned() {
        port
    } else {
        error!("Failed to read port from config, using default port 3000");
        3000
    };

    // Define routes
    let message_routes = Router::new().route("/sms.aspx", get(get_sms_aspx));
    let app = Router::new().merge(message_routes);

    info!("Server starting");
    let addr: SocketAddr = match format!("0.0.0.0:{port:?}").parse() {
        Ok(addr) => addr,
        Err(err) => {
            error!("Failed to parse address {}", err);
            exit(1);
        }
    };
    match Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        Ok(()) => {
            info!("Listen at {}", &addr);
        }
        Err(err) => {
            error!("Can not start server {}", err);
            exit(1);
        }
    }

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    info!("Got signal shutdown");
}