use axum::{body::Body, http::Request, middleware::Next, response::Response};
use log::info;

pub async fn logger_middleware(request: Request<Body>, next: Next) -> Response {
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
    // debug!("{:?}", request);
    next.run(request).await
}
