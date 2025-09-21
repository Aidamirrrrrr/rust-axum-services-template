use axum::{Router, routing::{get, post, any}};
use crate::{
    config::Config, error::AppError, middleware::{cors::make_cors_layer, trace::make_trace_layer}, routes::{health::health_handler, register::register_handler}, state::AppState  
};

pub fn build_app(state: AppState, config: &Config) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/register", post(register_handler))
        .fallback(any(|| async { Err::<(), _>(AppError::NotFound("Route not found".into())) }))
        .with_state(state)
        .layer(make_trace_layer())
        .layer(make_cors_layer(&config.cors_allowed_origins)) 
}
