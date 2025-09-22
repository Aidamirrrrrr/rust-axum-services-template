mod config;
mod common {
    pub mod error;
    pub mod state;
}
mod middlewares {
    pub mod cors;
    pub mod trace;
}
mod routes {
    pub mod health;
    pub mod register;
}
use crate::middlewares::cors::make_cors_layer;
use crate::middlewares::trace::make_trace_layer;
use crate::routes::health::health_handler;
use crate::routes::register::register_handler;
use axum::Router;
use axum::routing::{any, get, post};
use common::error::AppError;
use common::state::AppState;
use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Semaphore;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    let config = Config::from_env();
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations");

    let max_parallel_registrations = num_cpus::get().max(1);
    let registration_semaphore = Arc::new(Semaphore::new(max_parallel_registrations));

    let state = AppState {
        db_pool,
        registration_semaphore,
    };

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/register", post(register_handler))
        .fallback(any(|| async {
            Err::<(), _>(AppError::NotFound("Route not found".into()))
        }))
        .with_state(state)
        .layer(make_trace_layer())
        .layer(make_cors_layer(&config.cors_allowed_origins));

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("Invalid host:port");

    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app).await.unwrap();
}
