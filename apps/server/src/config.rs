use serde::Deserialize;
use sword::prelude::config;

#[derive(Debug, Deserialize)]
#[config(key = "postgres-db")]
pub struct PostgresDbConfig {
    pub url: String,
    pub migration_path: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub acquire_timeout_ms: u64,
}

// #[derive(Debug, Deserialize)]
// #[config(key = "auth")]
// pub struct AuthConfig {
//     pub session_jwt_secret: String,
//     pub session_jwt_exp_ms: u32,
//     pub refresh_jwt_secret: String,
//     pub refresh_jwt_exp_ms: u32,
// }

#[derive(Debug, Deserialize)]
#[config(key = "cors")]
pub struct CorsConfig {
    pub allow_credentials: bool,
    pub allowed_http_methods: Vec<String>,
    pub allowed_http_headers: Vec<String>,
}
