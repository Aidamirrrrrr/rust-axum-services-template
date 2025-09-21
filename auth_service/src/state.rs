use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub registration_semaphore: Arc<Semaphore>,
}
