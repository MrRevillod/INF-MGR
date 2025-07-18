use sword::prelude::Application;

use server::{
    asignatures::infrastructure::AsignaturesController,
    config::{CorsConfig, MailerConfig, PostgresDbConfig},
    shared::{
        database::PostgresDatabase,
        di::DependencyContainer,
        layers::{setup_cors, HttpLogger},
        smtp::LettreTransport,
    },
    users::infrastructure::UserController,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?;

    let cors_config = app.config.get::<CorsConfig>()?;
    let pg_db_config = app.config.get::<PostgresDbConfig>()?;
    let mailer_config = app.config.get::<MailerConfig>()?;

    let postgres_db = PostgresDatabase::new(&pg_db_config)
        .await
        .expect("Failed to create database connection");

    postgres_db
        .migrate()
        .await
        .expect("Failed to create database connection");

    let smtp_transport = LettreTransport::new(&mailer_config)
        .await
        .expect("Failed to create SMTP transport");

    let dependency_container = DependencyContainer::new(postgres_db, smtp_transport);

    let http_logger = HttpLogger::new();
    let cors_layer = setup_cors(&cors_config);

    app.di_module(dependency_container.module)?
        .controller::<UserController>()
        .controller::<AsignaturesController>()
        .layer(http_logger.layer)
        .layer(cors_layer)
        .run()
        .await?;

    Ok(())
}
