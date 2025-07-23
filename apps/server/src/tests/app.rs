use axum_test::TestServer;
use serde_json::Value;
use sword::prelude::Application;

use crate::{
    asignatures::infrastructure::AsignaturesController,
    config::{MailerConfig, PostgresDbConfig},
    inscriptions::infrastructure::InscriptionController,
    shared::{
        database::PostgresDatabase, di::DependencyContainer, smtp::LettreTransport,
    },
    users::infrastructure::UserController,
};

pub async fn init_test_app() -> TestServer {
    let app = Application::builder().expect("Failed to build Application");

    let pg_db_config = app.config.get::<PostgresDbConfig>().unwrap();
    let mailer_config = app.config.get::<MailerConfig>().unwrap();

    let postgres_db = PostgresDatabase::new(&pg_db_config)
        .await
        .expect("Failed to create database connection");

    postgres_db
        .migrate()
        .await
        .expect("Failed to migrate database");

    sqlx::query("TRUNCATE users, asignatures, inscriptions, practices CASCADE")
        .execute(&postgres_db.pool)
        .await
        .expect("Failed to truncate database tables");

    let smtp_transport = LettreTransport::new(&mailer_config)
        .await
        .expect("Failed to create SMTP transport");

    let dependency_container = DependencyContainer::new(postgres_db, smtp_transport);

    let app = app
        .di_module(dependency_container.module)
        .expect("Failed to load DI module")
        .controller::<UserController>()
        .controller::<AsignaturesController>()
        .controller::<InscriptionController>();

    TestServer::new(app.router()).expect("Failed to start test server")
}

pub fn extract_resource_id(data: &Value) -> String {
    data.get("id")
        .and_then(|id| id.as_str())
        .map(String::from)
        .unwrap_or_else(|| panic!("Response does not contain 'id': {:?}", data))
}
