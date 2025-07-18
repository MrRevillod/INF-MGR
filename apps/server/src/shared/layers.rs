use std::collections::HashSet;
use std::str::FromStr;
use std::time::Duration;

use axum::http;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::cors::CorsLayer;
use tower_http::trace::{
    MakeSpan, OnRequest, OnResponse, TraceLayer as TowerTraceLayer,
};

use tracing::Span;

use crate::config::CorsConfig;

type TraceLayer = TowerTraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    TraceMakeSpan,
    TraceOnRequest,
    TraceOnResponse,
>;

#[derive(Clone, Debug)]
pub struct HttpLogger {
    pub layer: TraceLayer,
}

impl HttpLogger {
    pub fn new() -> Self {
        tracing_subscriber::fmt()
            .with_target(false)
            .compact()
            .init();

        HttpLogger {
            layer: TowerTraceLayer::new_for_http()
                .make_span_with(TraceMakeSpan)
                .on_request(TraceOnRequest)
                .on_response(TraceOnResponse),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TraceMakeSpan;

impl<B> MakeSpan<B> for TraceMakeSpan {
    fn make_span(&mut self, _: &http::Request<B>) -> Span {
        tracing::info_span!("request")
    }
}

#[derive(Clone, Debug)]
pub struct TraceOnRequest;

impl<B> OnRequest<B> for TraceOnRequest {
    fn on_request(&mut self, request: &http::Request<B>, _: &Span) {
        tracing::info!(
            "HTTP - METHOD: [{}] - PATH: [{}]",
            request.method(),
            request.uri().path()
        );
    }
}

#[derive(Clone, Debug)]
pub struct TraceOnResponse;

impl<B> OnResponse<B> for TraceOnResponse {
    fn on_response(self, response: &http::Response<B>, latency: Duration, _: &Span) {
        tracing::info!(
            "HTTP - STATUS: [{}] - LATENCY: [{}ms]",
            response.status().as_u16(),
            latency.as_millis()
        );
        println!();
    }
}

impl Default for HttpLogger {
    fn default() -> Self {
        Self::new()
    }
}

pub fn setup_cors(config: &CorsConfig) -> CorsLayer {
    let mut methods = HashSet::new();
    let mut headers = HashSet::new();

    for method in config.allowed_http_methods.iter() {
        let http_method = axum::http::Method::from_str(method)
            .expect("Invalid HTTP Method found in config");

        methods.insert(http_method);
    }

    for header in config.allowed_http_headers.iter() {
        let http_header = axum::http::header::HeaderName::from_str(header)
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
