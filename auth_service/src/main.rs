mod config;
mod db;
mod error;
mod state;
mod middleware { 
    pub mod trace;
    pub mod cors;
}
mod routes {
    pub mod health;
    pub mod register;
}
mod logging;
mod app;

use std::{net::SocketAddr, sync::Arc};
use config::Config;
use db::create_pool;
use state::AppState;
use logging::init_tracing;
use app::build_app;
use tokio::sync::Semaphore;
use tracing::info;

#[tokio::main]
async fn main() {
    init_tracing();

    let config = Config::from_env();
    let db_pool = create_pool(&config.database_url).await;

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

    let app = build_app(state, &config);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("Invalid host:port");

    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app).await.unwrap();
}
