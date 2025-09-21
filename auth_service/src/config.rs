use std::env;

pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: String,
    pub cors_allowed_origins: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".into());

        let cors_allowed_origins =
            env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "*".into());

        Self {
            database_url,
            host,
            port,
            cors_allowed_origins,
        }
    }
}
