use std::time::Duration;
use sword::__internal::{AxumBody, AxumRequest, AxumResponse};

use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{MakeSpan, OnRequest, OnResponse, TraceLayer};
use tracing::Span;
use tracing_subscriber::fmt;

#[allow(non_snake_case)]
pub fn LoggerLayer() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    impl MakeSpan<AxumBody> + Clone,
    impl OnRequest<AxumBody> + Clone,
    impl OnResponse<AxumBody> + Clone,
> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_span_events(fmt::format::FmtSpan::NONE)
        .compact()
        .init();

    TraceLayer::new_for_http()
        .on_request(|req: &AxumRequest, _: &Span| {
            tracing::info!(
                "HTTP - METHOD: [{}] - PATH: [{}]",
                req.method(),
                req.uri().path()
            );
        })
        .on_response(|res: &AxumResponse, latency: Duration, _: &Span| {
            tracing::info!(
                "HTTP - STATUS: [{}] - LATENCY: [{}ms]",
                res.status().as_u16(),
                latency.as_millis()
            );
        })
}
