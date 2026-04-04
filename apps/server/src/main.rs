use helmet::*;
use sword::prelude::*;
use tokio::sync::mpsc;

use server::{
    auth::AuthModule,
    config::*,
    courses::CoursesModule,
    enrollments::EnrollmentsModule,
    imports::ImportsModule,
    logger::LoggerLayer,
    practices::PracticesModule,
    shared::{SharedModule, event_queue::*},
    users::UsersModule,
};

#[sword::main]
async fn main() {
    let mut app = Application::builder();

    let event_queue_config = app
        .config
        .get::<EventQueueConfig>()
        .expect("Failed to load EventQueueConfig");

    let (tx, rx) = mpsc::channel(event_queue_config.buffer_size);

    let event_queue = EventQueue::new(tx);
    let event_subscriber = EventSubscriber::builder()
        .with_receiver(rx)
        .with_config(app.config.clone())
        .build()
        .await;

    event_subscriber.run();

    app = app
        .with_provider(event_queue)
        .with_module::<SharedModule>()
        .with_module::<UsersModule>()
        .with_module::<CoursesModule>()
        .with_module::<EnrollmentsModule>()
        .with_module::<ImportsModule>()
        .with_module::<AuthModule>()
        .with_module::<PracticesModule>();

    app = app.with_layer(LoggerLayer()).with_layer(
        Helmet::builder()
            .with_header(XContentTypeOptions::nosniff())
            .with_header(XFrameOptions::same_origin())
            .with_header(StrictTransportSecurity::new().max_age(31_536_000))
            .with_header(CrossOriginResourcePolicy::same_origin())
            .with_header(ReferrerPolicy::strict_origin_when_cross_origin())
            .build(),
    );

    app.build().run().await;
}
