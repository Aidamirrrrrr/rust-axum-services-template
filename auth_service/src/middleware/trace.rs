use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tracing::Level;

pub type HttpTraceLayer = TraceLayer<SharedClassifier<ServerErrorsAsFailures>>;

pub fn make_trace_layer() -> HttpTraceLayer {
    TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .include_headers(false),
        )
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO),
        )
}
