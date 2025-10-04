use serde::Deserialize;
use shaku::{Component, Interface};
use sword::core::{Config, config};

#[derive(Debug, Deserialize)]
#[config(key = "application")]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub client_app_url: String,
    pub documents_dir: String,
    pub students_api_url: String,
}

#[derive(Debug, Deserialize)]
#[config(key = "students-api")]
pub struct StudentsApiConfig {
    pub api_key: String,
    pub api_url: String,
}

#[derive(Debug, Deserialize, Default)]
#[config(key = "google-calendar")]
pub struct GoogleCalendarConfig {
    pub calendar_id: String,
    pub service_account_path: String,
}

#[derive(Debug, Deserialize)]
#[config(key = "event-queue")]
pub struct EventQueueConfig {
    pub buffer_size: usize,
    pub num_of_event_retry: u8,
    pub delay_between_event_retry_ms: u64,
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

#[derive(Deserialize)]
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

// ---------------- Config Service ---------------------------

pub trait ConfigService: Interface {
    fn inner(&self) -> &Config;
}

#[derive(Component)]
#[shaku(interface = ConfigService)]
pub struct ConfigServiceImpl {
    inner: Config,
}

impl ConfigService for ConfigServiceImpl {
    fn inner(&self) -> &Config {
        &self.inner
    }
}

impl ConfigServiceImpl {
    pub fn new(config: Config) -> Self {
        Self { inner: config }
    }
}

impl From<ConfigServiceImpl> for ConfigServiceImplParameters {
    fn from(value: ConfigServiceImpl) -> Self {
        Self {
            inner: value.inner.clone(),
        }
    }
}
