use std::collections::HashSet;
use std::str::FromStr;
use std::time::Duration;

use sword::web::Method;
use sword::web::header::HeaderName;

use axum::http;
use tower_http::cors::CorsLayer;
use tower_http::trace::{MakeSpan, OnRequest, OnResponse, TraceLayer};
use tracing_subscriber::fmt;

use crate::config::CorsConfig;
use tracing::Span;

#[allow(non_snake_case)]
pub fn LoggerLayer() -> TraceLayer<
    tower_http::classify::SharedClassifier<
        tower_http::classify::ServerErrorsAsFailures,
    >,
    impl MakeSpan<axum::body::Body> + Clone,
    impl OnRequest<axum::body::Body> + Clone,
    impl OnResponse<axum::body::Body> + Clone,
> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_span_events(fmt::format::FmtSpan::NONE)
        .compact()
        .init();

    TraceLayer::new_for_http()
        .on_request(|req: &http::Request<_>, _: &Span| {
            tracing::info!(
                "HTTP - METHOD: [{}] - PATH: [{}]",
                req.method(),
                req.uri().path()
            );
        })
        .on_response(|res: &http::Response<_>, latency: Duration, _: &Span| {
            tracing::info!(
                "HTTP - STATUS: [{}] - LATENCY: [{}ms]",
                res.status().as_u16(),
                latency.as_millis()
            );
        })
}

#[allow(non_snake_case)]
pub fn CorsLayer(config: &CorsConfig) -> CorsLayer {
    let mut methods = HashSet::new();
    let mut headers = HashSet::new();

    for method in config.allowed_http_methods.iter() {
        let http_method =
            Method::from_str(method).expect("Invalid HTTP Method found in config");

        methods.insert(http_method);
    }

    for header in config.allowed_http_headers.iter() {
        let http_header = HeaderName::from_str(header)
            .expect("Invalid HTTP Header found in config");

        headers.insert(http_header);
    }

    let methods = methods.into_iter().collect::<Vec<_>>();
    let headers = headers.into_iter().collect::<Vec<_>>();

    CorsLayer::new()
        .allow_credentials(config.allow_credentials)
        .allow_methods(methods)
        .allow_headers(headers)
}
