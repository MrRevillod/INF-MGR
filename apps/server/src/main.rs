use helmet::*;
use sword::core::Config;
use sword::prelude::*;
use tokio::sync::mpsc;

use server::{
    auth::AuthController,
    config::*,
    courses::CoursesController,
    enrollments::EnrollmentsController,
    meetings::MeetingsController,
    shared::{services::*, *},
    users::UsersController,
};

#[sword::main]
async fn main() {
    let mut app = Application::builder()?;

    let config = app.config.clone();
    let event_queue_config = config.get::<EventQueueConfig>()?;

    let (
        pg_db,
        mailer,
        printer,
        oauth_client,
        redis_db,
        calendar_hub,
        config_service,
    ) = build_initial_components(config.clone())
        .await
        .expect("Failed to build dependencies");

    let (tx, rx) = mpsc::channel(event_queue_config.buffer_size);

    let dependency_container = DependencyContainer::builder()
        .with_postgres_db(pg_db)
        .with_event_queue(TokioEventQueue::new(tx))
        .with_oauth_client(oauth_client)
        .with_redis_db(redis_db)
        .with_calendar_hub(calendar_hub)
        .with_config_service(config_service)
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
        .with_controller::<AuthController>()
        .with_controller::<MeetingsController>()
        .with_layer(LoggerLayer())
        .with_layer(CorsLayer(&config.get::<CorsConfig>()?))
        .with_layer(
            Helmet::builder()
                .with_header(XContentTypeOptions::nosniff())
                .with_header(XFrameOptions::same_origin())
                .with_header(StrictTransportSecurity::new().max_age(31536000))
                .with_header(CrossOriginResourcePolicy::same_origin())
                .with_header(ReferrerPolicy::strict_origin_when_cross_origin())
                .build(),
        );

    app.build().run().await?;
}

async fn build_initial_components(
    config: Config,
) -> Result<InitialComponents, Box<dyn std::error::Error>> {
    let mailer_config = config.get::<MailerConfig>()?;
    let template_config = config.get::<TemplateConfig>()?;
    let calendar_config = config.get::<GoogleCalendarConfig>()?;

    let pg_db = PostgresDatabase::new(&config.get::<PostgresDbConfig>()?).await?;

    pg_db.migrate().await?;

    let mailer = Mailer::new(&mailer_config, &template_config)?;
    let printer = Printer::new(&template_config)?;
    let oauth_client = GoogleOAuthClient::new(&config.get::<AuthConfig>()?);
    let redis_db = RedisDatabase::new(&config.get::<RedisConfig>()?).await?;

    let google_calendar_hub = CalendarHub::new(
        &calendar_config.service_account_path,
        &calendar_config.calendar_id,
    )
    .await;

    Ok((
        pg_db,
        mailer,
        printer,
        oauth_client,
        redis_db,
        google_calendar_hub,
        ConfigServiceImpl::new(config),
    ))
}
