use axum_helmet::{Helmet, HelmetLayer};
use sword::prelude::Application;
use tokio::sync::mpsc;

use server::{
    config::ApplicationConfig,
    courses::CoursesController,
    enrollments::EnrollmentsController,
    shared::{
        database::PostgresDatabase,
        layers::{setup_cors, HttpLogger},
        services::event_queue::SubscriberOptions,
    },
    users::UsersController,
};

use server::shared::services::{
    event_queue::{EventSubscriber, TokioEventSender},
    mailer::{Mailer, MailerConfig},
    printer::Printer,
    templates::TemplateConfig,
};

use server::config::{CorsConfig, PostgresDbConfig};
use server::container::DependencyContainer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::builder()?;

    let app_config = app.config.get::<ApplicationConfig>()?;
    let cors_config = app.config.get::<CorsConfig>()?;
    let pg_db_config = app.config.get::<PostgresDbConfig>()?;
    let mailer_config = app.config.get::<MailerConfig>()?;
    let template_config = app.config.get::<TemplateConfig>()?;

    let (db, mailer, printer) = {
        let db = PostgresDatabase::new(&pg_db_config)
            .await
            .expect("Failed to create database connection");

        db.migrate().await.expect("Failed to create database connection");

        let mailer =
            Mailer::new(&mailer_config, &template_config).expect("Failed to create mailer");

        let printer = Printer::new(&template_config).expect("Failed to create printer");

        (db, mailer, printer)
    };

    let (tx, rx) = mpsc::channel(app_config.event_queue_buffer_size);

    let publisher = TokioEventSender::new(tx);
    let dependency_container = DependencyContainer::new(db, publisher);

    EventSubscriber::new(SubscriberOptions {
        rx,
        mailer,
        printer,
    })
    .run_parallel()
    .await;

    let http_logger = HttpLogger::new();
    let cors_layer = setup_cors(&cors_config);

    let helmet_layer = HelmetLayer::new(
        Helmet::new()
            .add(axum_helmet::XContentTypeOptions::nosniff())
            .add(axum_helmet::XFrameOptions::same_origin())
            .add(axum_helmet::StrictTransportSecurity::new().max_age(31536000))
            .add(axum_helmet::CrossOriginResourcePolicy::same_origin())
            .add(axum_helmet::ReferrerPolicy::strict_origin_when_cross_origin()),
    );

    app.di_module(dependency_container.module)?
        .controller::<UsersController>()
        .controller::<CoursesController>()
        .controller::<EnrollmentsController>()
        .layer(http_logger.layer)
        .layer(cors_layer)
        .layer(helmet_layer)
        .run()
        .await?;

    Ok(())
}
