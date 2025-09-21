use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

pub async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    if let Err(err) = sqlx::query("SELECT 1").execute(&state.db_pool).await {
        let resp = HealthResponse {
            status: "error".into(),
            message: format!("Database unreachable: {}", err),
        };
        return (StatusCode::SERVICE_UNAVAILABLE, Json(resp));
    }

    let resp = HealthResponse {
        status: "ok".into(),
        message: "Service is healthy".into(),
    };
    (StatusCode::OK, Json(resp))
}
