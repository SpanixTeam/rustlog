use axum::{
    extract::{MatchedPath, Request},
    response::Response,
};
use std::time::Duration;
use tracing::{info, info_span, Span};

pub fn make_span_with(request: &Request) -> Span {
    let method = request.method().to_string();
    let url = request.uri().to_string();
    let path = match request.extensions().get::<MatchedPath>() {
        Some(path) => path.as_str(),
        None => request.uri().path(),
    };

    info_span!(
        "http-request",
        "http.method" = method.as_str(),
        "http.uri" = url.as_str(),
        "http.path" = path,
    )
}

pub fn on_response(response: &Response, latency: Duration, span: &Span) {
    let status = response.status();
    let ms = latency.as_millis();

    span.record("http.status", status.as_str());
    span.record("http.latency", ms.to_string().as_str());

    info!("HTTP response {status} processed in {ms}ms");
}
