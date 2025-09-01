use axum_test::TestServer;
use serde_json::Value;

use server::{
    config::{ApplicationConfig, AuthConfig, CorsConfig, RedisConfig},
    imports::ImportsController,
    shared::{
        layers::{CorsLayer, HelmetLayer, LoggerLayer},
        oauth::GoogleOAuthClient,
        redis::RedisDatabase,
        services::{
            event_queue::{EventSubscriber, SubscriberOptions, TokioEventSender},
            mailer::{Mailer, MailerConfig},
            printer::Printer,
            templates::TemplateConfig,
        },
    },
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

#[cfg(test)]
pub mod imports;

#[cfg(test)]
pub static TEST_EMAILS: std::sync::LazyLock<
    std::collections::HashMap<String, String>,
> = std::sync::LazyLock::new(|| {
    let mut m = std::collections::HashMap::new();

    if let Ok(student_email) = std::env::var("TEST_STUDENT_EMAIL") {
        m.insert("student".to_string(), student_email);
    }

    if let Ok(teacher_email) = std::env::var("TEST_TEACHER_EMAIL") {
        m.insert("teacher".to_string(), teacher_email);
    }

    if let Ok(supervisor_email) = std::env::var("TEST_SUPERVISOR_EMAIL") {
        m.insert("supervisor".to_string(), supervisor_email);
    }

    m
});

use server::{
    config::PostgresDbConfig, courses::CoursesController,
    enrollments::EnrollmentsController, shared::database::PostgresDatabase,
    shared::di::DependencyContainer, users::UsersController,
};

use tokio::sync::mpsc;

pub async fn init_test_app() -> Result<TestServer, Box<dyn std::error::Error>> {
    let mut app = Application::builder().expect("Failed to build application");
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
        .with_controller::<ImportsController>()
        .with_layer(LoggerLayer())
        .with_layer(CorsLayer(&config.get::<CorsConfig>()?))
        .with_layer(HelmetLayer());

    Ok(TestServer::new(app.build().router()).expect("Failed to start test server"))
}

pub fn extract_resource_id(data: &Value) -> String {
    data.get("id")
        .and_then(|id| id.as_str())
        .map(String::from)
        .unwrap_or_else(|| panic!("Response does not contain 'id': {data:?}"))
}
