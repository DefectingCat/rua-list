use std::{fmt::Display, time::Duration};

use axum::{
    body::Bytes,
    http::{HeaderMap, HeaderValue, Request},
    response::Response,
    Router,
};
use log::info;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{error, info_span, Span};

// pub async fn logger_middleware(request: Request<Body>, next: Next) -> Response {
//     let host = if let Some(host) = request.headers().get("host") {
//         host.to_str().unwrap_or("Unknown")
//     } else {
//         "Unknown"
//     };
//     info!(
//         "{} - {} - {:?} - {:?}",
//         request.method(),
//         request.uri(),
//         request.version(),
//         host
//     );
//     // debug!("{:?}", request);
//     next.run(request).await
// }

/// Format request latency and status message
/// return a string
fn format_latency(latency: Duration, status: impl Display) -> String {
    let micros = latency.as_micros();
    let millis = latency.as_millis();
    if micros >= 1000 {
        format!("{status} {millis}ms")
    } else {
        format!("{status} {micros}μs")
    }
}

/// Middleware for logging each request.
///
/// This middleware will calculate each request latency
/// and add request's information to each info_span.
pub fn logging_route(router: Router) -> Router {
    let make_span = |req: &Request<_>| {
        let unknown = &HeaderValue::from_static("Unknown");
        let empty = &HeaderValue::from_static("");
        let headers = req.headers();
        let ua = headers
            .get("User-Agent")
            .unwrap_or(unknown)
            .to_str()
            .unwrap_or("Unknown");
        let host = headers.get("Host").unwrap_or(empty).to_str().unwrap_or("");
        info_span!("HTTP", method = ?req.method(), host, uri = ?req.uri(), ua)
    };

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(make_span)
        .on_request(|_req: &Request<_>, _span: &Span| {})
        .on_response(|res: &Response, latency: Duration, _span: &Span| {
            info!("{}", format_latency(latency, res.status()));
        })
        .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {})
        .on_eos(|_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {})
        .on_failure(
            |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                error!("{}", format_latency(latency, error));
            },
        );

    router.layer(trace_layer)
}
