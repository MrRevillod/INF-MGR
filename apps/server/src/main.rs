use sword::prelude::Application;
use tokio::sync::mpsc;

use server::{
    config::*, courses::CoursesController, enrollments::EnrollmentsController,
    shared::redis::RedisDatabase, users::UsersController,
};

use server::shared::{
    database::PostgresDatabase,
    di::DependencyContainer,
    layers::{CorsLayer, HelmetLayer, LoggerLayer},
    oauth::GoogleOAuthClient,
};

use server::shared::services::{
    event_queue::*,
    mailer::{Mailer, MailerConfig},
    printer::Printer,
    templates::TemplateConfig,
};

#[sword::main]
async fn main() {
    let mut app = Application::builder()?;
    let config = app.config.clone();

    let (pg_db, mailer, printer, oauth_client, redis_db) = {
        let pg_db = PostgresDatabase::new(&config.get::<PostgresDbConfig>()?)
            .await
            .expect("Failed to create database connection");

        pg_db.migrate().await.expect("Failed to create database connection");

        let mailer_config = config.get::<MailerConfig>()?;
        let template_config = config.get::<TemplateConfig>()?;

        let mailer = Mailer::new(&mailer_config, &template_config)
            .expect("Failed to create mailer");

        let printer =
            Printer::new(&template_config).expect("Failed to create printer");

        let oauth_client = GoogleOAuthClient::new(&config.get::<AuthConfig>()?);

        let redis_db = RedisDatabase::new(&config.get::<RedisConfig>()?)
            .await
            .expect("Failed to create Redis connection");

        (pg_db, mailer, printer, oauth_client, redis_db)
    };

    let app_config = config.get::<ApplicationConfig>()?;

    let (tx, rx) = mpsc::channel(app_config.event_queue_buffer_size);

    let publisher = TokioEventSender::new(tx);
    let dependency_container = DependencyContainer::builder()
        .with_postgres_db(pg_db)
        .with_event_sender(publisher)
        .with_oauth_client(oauth_client)
        .with_redis_db(redis_db)
        .build();

    EventSubscriber::new(SubscriberOptions {
        rx,
        mailer,
        printer,
    })
    .run_parallel()
    .await;

    app = app
        .with_shaku_di_module(dependency_container.module)?
        .with_controller::<UsersController>()
        .with_controller::<CoursesController>()
        .with_controller::<EnrollmentsController>()
        .with_layer(LoggerLayer())
        .with_layer(CorsLayer(&config.get::<CorsConfig>()?))
        .with_layer(HelmetLayer());

    app.build().run().await?;
}
