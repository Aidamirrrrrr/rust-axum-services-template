use tower_http::cors::{CorsLayer, Any, AllowOrigin};
use axum::http;

pub fn make_cors_layer(origins: &str) -> CorsLayer {
    if origins.trim() == "*" {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS])
            .allow_headers([http::header::CONTENT_TYPE])
    } else {
        let origins_vec: Vec<_> = origins
            .split(',')
            .filter_map(|o| o.trim().parse().ok())
            .collect();

        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins_vec))
            .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS])
            .allow_headers([http::header::CONTENT_TYPE])
    }
}
