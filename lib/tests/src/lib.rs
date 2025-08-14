use axum_test::TestServer;
use serde_json::Value;
use services::{
    event_queue::{EventSubscriber, SubscriberServices, TokioEventSender},
    mailer::{Mailer, MailerConfig},
    printer::Printer,
    templates::TemplateConfig,
};
use sword::prelude::Application;

#[cfg(test)]
pub mod courses;
#[cfg(test)]
pub mod enrollments;
#[cfg(test)]
pub mod practices;
#[cfg(test)]
pub mod users;

use server::{
    config::PostgresDbConfig, container::DependencyContainer,
    courses::CoursesController, enrollments::EnrollmentsController,
    shared::database::PostgresDatabase, users::UsersController,
};
use tokio::sync::mpsc;

pub async fn init_test_app() -> TestServer {
    let mut app =
        Application::builder().expect("Failed to create application builder");

    let pg_db_config = app
        .config
        .get::<PostgresDbConfig>()
        .expect("Failed to get PostgresDbConfig");

    let mailer_config = app
        .config
        .get::<MailerConfig>()
        .expect("Failed to get MailerConfig");

    let tamplate_config = app
        .config
        .get::<TemplateConfig>()
        .expect("Failed to get TemplateConfig");

    let (db, mailer, printer) = {
        let db = PostgresDatabase::new(&pg_db_config)
            .await
            .expect("Failed to create database connection");

        db.migrate()
            .await
            .expect("Failed to create database connection");

        let mailer = Mailer::new(&mailer_config, &tamplate_config)
            .expect("Failed to create mailer");

        let printer =
            Printer::new(&tamplate_config).expect("Failed to create printer");

        (db, mailer, printer)
    };

    let (tx, rx) = mpsc::channel(100);

    let publisher = TokioEventSender::new(tx);
    let dependency_container = DependencyContainer::new(db, publisher);

    let sub_queue = EventSubscriber::new(rx, SubscriberServices { mailer, printer });

    tokio::spawn(async move {
        if let Err(e) = sub_queue.subscribe().await {
            eprintln!("Error in event subscriber: {}", e);
        }
    });

    app = app
        .di_module(dependency_container.module)
        .expect("Failed to load dependency module")
        .controller::<UsersController>()
        .controller::<CoursesController>()
        .controller::<EnrollmentsController>();

    TestServer::new(app.router()).expect("Failed to start test server")
}

pub fn extract_resource_id(data: &Value) -> String {
    dbg!(data);
    data.get("id")
        .and_then(|id| id.as_str())
        .map(String::from)
        .unwrap_or_else(|| panic!("Response does not contain 'id': {data:?}"))
}
