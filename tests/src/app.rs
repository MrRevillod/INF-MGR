use axum_test::TestServer;
use server::{
    auth::{JsonWebTokenService, TokenConfig},
    config::{
        AuthConfig, CorsConfig, EventQueueConfig, PostgresDbConfig, RedisConfig,
    },
    courses::CoursesController,
    enrollments::EnrollmentsController,
    imports::ImportsController,
    shared::{
        CorsLayer, DependencyContainer, GoogleOAuthClient, InitialComponents,
        PostgresDatabase, RedisDatabase, init_tracing,
        services::{
            EventSubscriber, Mailer, MailerConfig, Printer, TemplateConfig,
            TokioEventQueue,
        },
    },
    users::UsersController,
};
use sword::{core::Config, prelude::*};
use tokio::sync::mpsc;

pub async fn init_test_app() -> Result<TestServer, Box<dyn std::error::Error>> {
    init_tracing();

    let mut app = Application::builder().expect("Failed to build application");
    let config = app.config.clone();
    let event_queue_config = config.get::<EventQueueConfig>()?;

    let (pg_db, mailer, printer, oauth_client, redis_db, jsonwebtoken_service) =
        build_initial_components(config.clone())
            .await
            .expect("Failed to build dependencies");

    let (tx, rx) = mpsc::channel(event_queue_config.buffer_size);

    let dependency_container = DependencyContainer::builder()
        .with_postgres_db(pg_db)
        .with_jwt_service(jsonwebtoken_service)
        .with_event_queue(TokioEventQueue::new(tx))
        .with_oauth_client(oauth_client)
        .with_redis_db(redis_db)
        .build();

    let event_subscriber = EventSubscriber::builder()
        .with_receiver(rx)
        .with_mailer(mailer)
        .with_printer(printer)
        .with_config(event_queue_config)
        .build();

    event_subscriber.run_parallel().await;

    app = app
        .with_shaku_di_module(dependency_container)?
        .with_controller::<UsersController>()
        .with_controller::<CoursesController>()
        .with_controller::<EnrollmentsController>()
        .with_controller::<ImportsController>()
        .with_layer(CorsLayer(&config.get::<CorsConfig>()?));

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

    pg_db.migrate().await.expect("Failed to run migrations");

    sqlx::query("TRUNCATE TABLE users, courses, enrollments, practices CASCADE")
        .execute(&pg_db.pool)
        .await?;

    let mailer = Mailer::new(&mailer_config, &template_config)
        .expect("Failed to create mailer");
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

    Ok((
        pg_db,
        mailer,
        printer,
        oauth_client,
        redis_db,
        jsonwebtoken_service,
    ))
}
