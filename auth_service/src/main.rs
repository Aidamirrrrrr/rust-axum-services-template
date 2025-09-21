mod config;
mod db;
mod state;
mod routes {
    pub mod health;
}

use axum::{routing::get, Router};
use std::net::SocketAddr;

use config::Config;
use db::create_pool;
use state::AppState;
use routes::health::health_handler;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let db_pool = create_pool(&config.database_url).await;
    let state = AppState { db_pool };

    let app = Router::new()
        .route("/health", get(health_handler))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("Invalid host:port");

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app).await.unwrap();
}
