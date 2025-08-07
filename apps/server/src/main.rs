use sword::prelude::Application;

use server::{
    courses::CoursesController,
    enrollments::EnrollmentsController,
    shared::{
        database::PostgresDatabase,
        layers::{setup_cors, HttpLogger},
    },
    users::UsersController,
};

use services::{
    mailer::{MailerConfig, MailerService},
    printer::DocumentPrinter,
    templates::TemplateConfig,
};

use server::config::{CorsConfig, PostgresDbConfig};
use server::container::DependencyContainer;

pub const DEFAULT_PAGE_SIZE: usize = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?;

    let cors_config = app.config.get::<CorsConfig>()?;
    let pg_db_config = app.config.get::<PostgresDbConfig>()?;
    let mailer_config = app.config.get::<MailerConfig>()?;
    let tamplate_config = app.config.get::<TemplateConfig>()?;

    let postgres_db = PostgresDatabase::new(&pg_db_config)
        .await
        .expect("Failed to create database connection");

    postgres_db
        .migrate()
        .await
        .expect("Failed to create database connection");

    let smtp_transport = MailerService::new(&mailer_config, &tamplate_config)
        .expect("Failed to create Mailer service");

    let printer = DocumentPrinter::new(&tamplate_config)
        .expect("Failed to create DocumentPrinter service");

    let dependency_container =
        DependencyContainer::new(postgres_db, smtp_transport, printer);

    let http_logger = HttpLogger::new();
    let cors_layer = setup_cors(&cors_config);

    app.di_module(dependency_container.module)?
        .controller::<UsersController>()
        .controller::<CoursesController>()
        .controller::<EnrollmentsController>()
        .layer(http_logger.layer)
        .layer(cors_layer)
        .run()
        .await?;

    Ok(())
}
