use std::{net::SocketAddr, process::exit, time::Duration};

use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer,
    http,
    http::StatusCode,
    middleware::{self},
    response,
    routing::get,
    BoxError, Router, Server,
};
use log::{error, info};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    net::{TcpListener, TcpStream},
};
use tower::{timeout::TimeoutLayer, ServiceBuilder};

use crate::{
    config::Config,
    middlewares::{headers_parse::MyLayer, logger::logger_middleware},
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
        .fallback(fallback)
        .with_state(config.list)
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(logger_middleware))
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(MyLayer)
                .layer(TimeoutLayer::new(Duration::from_secs(10))),
        );

    tokio::spawn(async move {
        let addr: SocketAddr = match format!("0.0.0.0:{:?}", port).parse() {
            Ok(addr) => addr,
            Err(err) => {
                error!("Failed to parse address {}", err);
                exit(1);
            }
        };
        let listener = TcpListener::bind(addr).await.expect("Can not start server");

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                let (mut reader, mut writer) = socket.split();

                let mut buf = BufReader::new(reader);
                let mut header = String::new();
                loop {
                    let count = buf.read_line(&mut header).await.unwrap();
                    if count < 3 {
                        break;
                    }
                }
                let mut body = String::new();
                loop {
                    let count = buf.read_line(&mut body).await.unwrap();
                    if count < 3 {
                        break;
                    }
                }
                let header: Vec<_> = header.split("\r\n").collect();
                let first_line = header.first().unwrap();
                let header = &header[1..header.len() - 2];
                let header: Vec<_> = header
                    .iter()
                    .filter(|head| head.contains(':'))
                    .map(|head| head.to_string())
                    .collect();
                let headers = format!("{first_line}\r\n{}\r\n\r\n", header.join(""));
                dbg!(&headers, &body);
            });
        }
    });

    let addr: SocketAddr = match format!("0.0.0.0:{:?}", port + 1).parse() {
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