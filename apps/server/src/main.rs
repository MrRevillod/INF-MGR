use sword::core::Config;
use sword::prelude::Application;
use tokio::sync::mpsc;

use server::{
    auth::{AuthController, JsonWebTokenService, TokenConfig},
    config::*,
    courses::CoursesController,
    enrollments::EnrollmentsController,
    users::UsersController,
};

use server::shared::{
    database::PostgresDatabase,
    di::{DependencyContainer, InitialComponents},
    layers::{CorsLayer, HelmetLayer, LoggerLayer},
    oauth::GoogleOAuthClient,
    redis::RedisDatabase,
    services::{
        event_queue::*,
        mailer::{Mailer, MailerConfig},
        printer::Printer,
        templates::TemplateConfig,
    },
};

#[sword::main]
async fn main() {
    let mut app = Application::builder()?;

    let config = app.config.clone();
    let app_config = config.get::<ApplicationConfig>().expect("Invalid app config");

    let (pg_db, mailer, printer, oauth_client, redis_db, jsonwebtoken_service) =
        build_initial_components(config.clone())
            .await
            .expect("Failed to build dependencies");

    let (tx, rx) = mpsc::channel(app_config.event_queue_buffer_size);

    let dependency_container = DependencyContainer::builder()
        .with_postgres_db(pg_db)
        .with_event_queue(TokioEventQueue::new(tx))
        .with_oauth_client(oauth_client)
        .with_redis_db(redis_db)
        .with_jwt_service(jsonwebtoken_service)
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
        .with_controller::<AuthController>()
        .with_layer(LoggerLayer())
        .with_layer(CorsLayer(&config.get::<CorsConfig>()?))
        .with_layer(HelmetLayer());

    app.build().run().await?;
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
