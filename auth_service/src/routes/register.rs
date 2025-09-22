use crate::common::{error::AppError, state::AppState};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use std::{sync::Arc, time::Duration};
use tokio::{sync::OwnedSemaphorePermit, task, time};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: i32,
    pub username: String,
}

async fn acquire_permit_with_timeout(
    semaphore: Arc<tokio::sync::Semaphore>,
) -> Result<OwnedSemaphorePermit, AppError> {
    let timeout_dur = Duration::from_secs(10); // TODO: may be overridden in config
    match time::timeout(timeout_dur, semaphore.acquire_owned()).await {
        Ok(permit_res) => {
            permit_res.map_err(|_| AppError::Internal("Service is shutting down".into()))
        }
        Err(_) => Err(AppError::Internal("Server is busy, try later".into())),
    }
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    if payload.username.trim().is_empty() {
        return Err(AppError::BadRequest("Username cannot be empty".into()));
    }

    let permit = acquire_permit_with_timeout(state.registration_semaphore.clone()).await?;

    let password = payload.password.clone();

    let password_hash = task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|_| "Hashing failed".to_string())
    })
    .await
    .map_err(|_| AppError::Internal("Hashing task panicked".into()))?
    .map_err(|_| AppError::Internal("Password hashing failed".into()))?;

    drop(permit);

    let row = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id, username
        "#,
        payload.username,
        password_hash
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e
            && db_err.constraint() == Some("users_username_key")
        {
            return AppError::Conflict("Username already exists".into());
        }
        AppError::Internal("Database error".into())
    })?;

    let resp = RegisterResponse {
        id: row.id,
        username: row.username,
    };

    Ok((StatusCode::CREATED, axum::Json(resp)))
}
