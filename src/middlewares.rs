use std::fmt::Debug;

use axum::{http::Request, middleware::Next, response::Response};
use log::{debug, info};

pub async fn logger_middleware<B>(request: Request<B>, next: Next<B>) -> Response
where
    B: Debug,
{
    let host = if let Some(host) = request.headers().get("host") {
        host.to_str().unwrap_or("Unknown")
    } else {
        "Unknown"
    };
    info!(
        "{} - {} - {:?} - {:?}",
        request.method(),
        request.uri(),
        request.version(),
        host
    );
    debug!("{:?}", request);
    next.run(request).await
}