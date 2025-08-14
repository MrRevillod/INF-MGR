use sword::prelude::Application;
use tokio::sync::mpsc;

use server::{
    courses::CoursesController,
    enrollments::EnrollmentsController,
    shared::{
        database::PostgresDatabase,
        layers::{setup_cors, HttpLogger},
    },
    users::UsersController,
};

use server::shared::services::{
    event_queue::{EventSubscriber, SubscriberServices, TokioEventSender},
    mailer::{Mailer, MailerConfig},
    printer::Printer,
    templates::TemplateConfig,
};

use server::config::{CorsConfig, PostgresDbConfig};
use server::container::DependencyContainer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?;

    let cors_config = app.config.get::<CorsConfig>()?;
    let pg_db_config = app.config.get::<PostgresDbConfig>()?;
    let mailer_config = app.config.get::<MailerConfig>()?;
    let template_config = app.config.get::<TemplateConfig>()?;

    let (db, mailer, printer) = {
        let db = PostgresDatabase::new(&pg_db_config)
            .await
            .expect("Failed to create database connection");

        db.migrate()
            .await
            .expect("Failed to create database connection");

        let mailer = Mailer::new(&mailer_config, &template_config)
            .expect("Failed to create mailer");

        let printer =
            Printer::new(&template_config).expect("Failed to create printer");

        (db, mailer, printer)
    };

    let (tx, rx) = mpsc::channel(100);

    let publisher = TokioEventSender::new(tx);
    let dependency_container = DependencyContainer::new(db, publisher);

    let sub_queue = EventSubscriber::new(rx, SubscriberServices { mailer, printer });

    tokio::spawn(async move {
        if let Err(e) = sub_queue.subscribe().await {
            tracing::error!("Error in event subscriber: {e}");
        }
    });

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
