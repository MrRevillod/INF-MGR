use axum_test::TestServer;
use serde_json::Value;

use server::{
    auth::{JsonWebTokenService, TokenConfig},
    config::{ApplicationConfig, AuthConfig, CorsConfig, RedisConfig},
    imports::ImportsController,
    shared::{
        di::InitialComponents,
        layers::{CorsLayer, HelmetLayer},
        oauth::GoogleOAuthClient,
        redis::RedisDatabase,
        services::{
            event_queue::{EventSubscriber, SubscriberOptions, TokioEventQueue},
            mailer::{Mailer, MailerConfig},
            printer::Printer,
            templates::TemplateConfig,
        },
    },
};

use sword::{core::Config, prelude::Application};

#[cfg(test)]
pub mod courses;
#[cfg(test)]
pub mod enrollments;
#[cfg(test)]
pub mod practices;
#[cfg(test)]
pub mod users;

#[cfg(test)]
pub static TEST_EMAILS: std::sync::LazyLock<std::collections::HashMap<String, String>> =
    std::sync::LazyLock::new(|| {
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
    config::PostgresDbConfig, courses::CoursesController, enrollments::EnrollmentsController,
    shared::database::PostgresDatabase, shared::di::DependencyContainer, users::UsersController,
};

use tokio::sync::mpsc;

pub async fn init_test_app() -> Result<TestServer, Box<dyn std::error::Error>> {
    let mut app = Application::builder().expect("Failed to build application");
    let config = app.config.clone();

    let (pg_db, mailer, printer, oauth_client, redis_db, jsonwebtoken_service) =
        build_initial_components(config.clone())
            .await
            .expect("Failed to build dependencies");

    let app_config = config.get::<ApplicationConfig>()?;

    let (tx, rx) = mpsc::channel(app_config.event_queue_buffer_size);

    sqlx::query("TRUNCATE TABLE users, courses, enrollments, practices CASCADE")
        .execute(&pg_db.pool)
        .await?;

    pg_db.migrate().await.expect("Failed to create database connection");

    let dependency_container = DependencyContainer::builder()
        .with_postgres_db(pg_db)
        .with_jwt_service(jsonwebtoken_service)
        .with_event_queue(TokioEventQueue::new(tx))
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
        .with_shaku_di_module(dependency_container)?
        .with_controller::<UsersController>()
        .with_controller::<CoursesController>()
        .with_controller::<EnrollmentsController>()
        .with_controller::<ImportsController>()
        .with_layer(CorsLayer(&config.get::<CorsConfig>()?))
        .with_layer(HelmetLayer());

    Ok(TestServer::new(app.build().router()).expect("Failed to start test server"))
}

async fn build_initial_components(
    config: Config,
) -> Result<InitialComponents, Box<dyn std::error::Error>> {
    let auth_config = config.get::<AuthConfig>()?;
    let mailer_config = config.get::<MailerConfig>()?;
    let template_config = config.get::<TemplateConfig>()?;

    let pg_db = PostgresDatabase::new(&config.get::<PostgresDbConfig>()?)
        .await
        .expect("Failed to create database connection");

    pg_db.migrate().await.expect("Failed to create database connection");

    let mailer = Mailer::new(&mailer_config, &template_config).expect("Failed to create mailer");
    let printer = Printer::new(&template_config).expect("Failed to create printer");

    let oauth_client = GoogleOAuthClient::new(&config.get::<AuthConfig>()?);

    let redis_db = RedisDatabase::new(&config.get::<RedisConfig>()?)
        .await
        .expect("Failed to create Redis connection");

    let jsonwebtoken_service = JsonWebTokenService::new(
        TokenConfig {
            secret: auth_config.access_jwt_secret,
            expiration: auth_config.access_exp_ms,
        },
        TokenConfig {
            secret: auth_config.refresh_jwt_secret,
            expiration: auth_config.refresh_exp_ms,
        },
    );

    Ok((pg_db, mailer, printer, oauth_client, redis_db, jsonwebtoken_service))
}

pub fn extract_resource_id(data: &Value) -> String {
    data.get("id")
        .and_then(|id| id.as_str())
        .map(String::from)
        .unwrap_or_else(|| panic!("Response does not contain 'id': {data:?}"))
}
