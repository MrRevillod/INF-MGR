use std::path::Path;

use config::{Config as Cfg, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct SessionsDbConfig {
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct PostgresDbConfig {
    pub url: String,
    pub migration_path: String,
    pub min_connections: u8,
    pub max_connections: u8,
    pub acquire_timeout_ms: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub postgres: PostgresDbConfig,
    pub sessions: SessionsDbConfig,
}

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    pub session_jwt_secret: String,
    pub session_jwt_exp_ms: u32,
    pub refresh_jwt_secret: String,
    pub refresh_jwt_exp_ms: u32,
}

#[derive(Debug, Deserialize)]
pub struct CorsConfig {
    pub allowed_http_methods: Vec<String>,
    pub allowed_http_headers: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub cors: CorsConfig,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let toml_path = Path::new("config/config.toml");
        let contents = std::fs::read_to_string(toml_path).map_err(|e| {
            config::ConfigError::Message(format!(
                "Failed to read config file: {}",
                e
            ))
        })?;

        let expanded = shellexpand::env(&contents)
            .map_err(|e| {
                config::ConfigError::Message(format!(
                    "Failed to expand environment variables: {}",
                    e
                ))
            })?
            .into_owned();

        Cfg::builder()
            .add_source(File::from_str(&expanded, config::FileFormat::Toml))
            .build()?
            .try_deserialize()
    }
}
