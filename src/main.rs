use std::{net::SocketAddr, process::exit};

use anyhow::Result;
use axum::{http, middleware, response, routing::get, Router, Server};
use log::{error, info};
use tower::ServiceBuilder;

use crate::{
    config::Config,
    middlewares::logger_middleware,
    routes::messages::{match_check_get, match_check_post},
};

mod arg;
mod config;
mod consts;
mod error;
mod http_client;
mod logger;
mod middlewares;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::build();

    if let Err(err) = logger::init_logger(&config).await {
        error!("Failed to create logger; {}", err.to_string());
        exit(1);
    }
    info!("Server starting");

    let port = if let Some(port) = config.port {
        port
    } else {
        error!("Failed to read port from config, using default port 3000");
        3000
    };

    // Define routes
    let message_routes = Router::new()
        .route("/sms.aspx", get(match_check_get).post(match_check_post))
        .route("/smsGBK.aspx", get(match_check_get).post(match_check_post));
    let app = Router::new()
        .merge(message_routes)
        .layer(ServiceBuilder::new().layer(middleware::from_fn(logger_middleware)))
        .fallback(fallback)
        .with_state(config.list);

    let addr: SocketAddr = match format!("0.0.0.0:{port:?}").parse() {
        Ok(addr) => addr,
        Err(err) => {
            error!("Failed to parse address {}", err);
            exit(1);
        }
    };
    info!("Server listening on {}", &addr);
    match Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        Ok(()) => {
            info!("Server shutdown");
        }
        Err(err) => {
            error!("Can not start server {}", err);
            exit(1);
        }
    }

    Ok(())
}

/// Handle server shutdown signal
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    info!("Got signal shutdown");
}

/// Response all fallback route with not found
pub async fn fallback(uri: http::Uri) -> impl response::IntoResponse {
    info!("Route {} not found", uri);
    (http::StatusCode::NOT_FOUND, "Not found")
}