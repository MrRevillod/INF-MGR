use crate::config::PostgresDbConfig;

use sqlx::{PgPool, Postgres, Transaction, postgres::PgPoolOptions};
use std::time::Duration;
use sword::core::injectable;

pub const DEFAULT_PAGE_SIZE: u64 = 10;

pub struct Pagination<T> {
    pub items: Vec<T>,
    pub current_page: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_previous: bool,
}

#[injectable(provider)]
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
        if let Err(e) = sqlx::migrate!("./config/migrations")
            .run(self.get_pool())
            .await
        {
            tracing::error!("Error running migrations: {e}");
        }

        tracing::info!("Database migrations applied successfully.");

        Ok(())
    }

    pub const fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn tx_begin(&self) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
        self.pool.begin().await
    }
}
