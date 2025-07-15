mod config;
mod features;
mod shared;

use sword::prelude::Application;

use crate::{
    config::{CorsConfig, PostgresDbConfig},
    shared::infrastructure::{
        setup_cors, DependencyContainer, HttpLogger, PostgresDatabase,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?;

    let cors_config = app.config.get::<CorsConfig>()?;
    let pg_db_config = app.config.get::<PostgresDbConfig>()?;

    let postgres_db = PostgresDatabase::new(&pg_db_config)
        .await
        .expect("Failed to create database connection");

    postgres_db
        .migrate()
        .await
        .expect("Failed to create database connection");

    let dependency_container = DependencyContainer::new(postgres_db);

    let http_logger = HttpLogger::new();
    let cors_layer = setup_cors(&cors_config);

    app.layer(http_logger.layer)
        .layer(cors_layer)
        .di_module(dependency_container.module)?
        .run()
        .await?;

    Ok(())
}
