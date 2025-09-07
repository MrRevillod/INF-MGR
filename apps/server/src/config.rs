use serde::Deserialize;
use sword::prelude::config;

#[derive(Debug, Deserialize)]
#[config(key = "application")]
pub struct ApplicationConfig {
    pub port: u16,
    pub host: String,
    pub event_queue_buffer_size: usize,
    pub client_app_url: String,
}

#[derive(Debug, Deserialize)]
#[config(key = "postgres-db")]
pub struct PostgresDbConfig {
    pub url: String,
    pub migration_path: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub acquire_timeout_ms: u64,
}

#[derive(Debug, Deserialize)]
#[config(key = "redis")]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[config(key = "auth")]
pub struct AuthConfig {
    pub access_jwt_secret: String,
    pub refresh_jwt_secret: String,

    pub access_exp_ms: usize,
    pub refresh_exp_ms: usize,

    pub session_ttl_seconds: u64,

    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
}

#[derive(Debug, Deserialize)]
#[config(key = "cors")]
pub struct CorsConfig {
    pub allow_credentials: bool,
    pub allowed_http_methods: Vec<String>,
    pub allowed_http_headers: Vec<String>,
}
