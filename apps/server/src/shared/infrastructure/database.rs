use std::time::Duration;

use shaku::{Component, Interface};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::PostgresDbConfig;

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
        if let Err(e) = sqlx::migrate!("./config/migrations")
            .run(self.get_pool())
            .await
        {
            eprintln!("Error running migrations: {}", e);
        };

        Ok(())
    }
}

impl DatabaseConnection for PostgresDatabase {
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl Into<PostgresDatabaseParameters> for PostgresDatabase {
    fn into(self) -> PostgresDatabaseParameters {
        PostgresDatabaseParameters { pool: self.pool }
    }
}
