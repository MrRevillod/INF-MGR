use crate::config::PostgresDbConfig;

use shaku::{Component, Interface};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub const DEFAULT_PAGE_SIZE: u64 = 10;

pub struct Pagination<T> {
    pub items: Vec<T>,
    pub current_page: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_previous: bool,
}

pub trait DatabaseConnection: Interface {
    fn get_pool(&self) -> &PgPool;
}

#[derive(Component)]
#[shaku(interface = DatabaseConnection)]
pub struct PostgresDatabase {
    pub pool: PgPool,
}

impl PostgresDatabase {
    pub async fn new(config: &PostgresDbConfig) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_millis(config.acquire_timeout_ms))
            .connect(&config.url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        if let Err(e) = sqlx::migrate!("./config/migrations").run(self.get_pool()).await {
            tracing::error!("Error running migrations: {e}");
        };

        Ok(())
    }
}

impl DatabaseConnection for PostgresDatabase {
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl From<PostgresDatabase> for PostgresDatabaseParameters {
    fn from(value: PostgresDatabase) -> Self {
        PostgresDatabaseParameters { pool: value.pool }
    }
}
