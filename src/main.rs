use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer, http, http::StatusCode, middleware, response, routing::get,
    BoxError, Router, Server,
};
use log::{error, info};
use std::{net::SocketAddr, process::exit, time::Duration};
use tower::{timeout::TimeoutLayer, ServiceBuilder};

use crate::{
    config::Config,
    // header_parser::headers_parser,
    logger::logger_init,
    middlewares::logger::logger_middleware,
    routes::messages::{match_check_get, match_check_post},
};

mod arg;
mod config;
mod consts;
mod error;
mod header_parser;
mod http_client;
mod logger;
mod middlewares;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::build();

    let _guard = logger_init(&config)?;
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
        .fallback(fallback)
        .with_state(config.list)
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(logger_middleware))
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(10))),
        );

    let addr: SocketAddr = match format!("0.0.0.0:{:?}", port).parse() {
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

    // headers_parser(port).await;
    Ok(())
}

/// Handle server shutdown signal
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    info!("Got signal shutdown");
    info!("Server shutdown");
    exit(1);
}

/// Response all fallback route with not found
pub async fn fallback(uri: http::Uri) -> impl response::IntoResponse {
    info!("Route {} not found", uri);
    (http::StatusCode::NOT_FOUND, "Not found")
}