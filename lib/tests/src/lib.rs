use axum_test::TestServer;
use serde_json::Value;
use sword::prelude::Application;

#[cfg(test)]
pub mod courses;
#[cfg(test)]
pub mod inscriptions;
#[cfg(test)]
pub mod users;

use server::{
    config::PostgresDbConfig, container::DependencyContainer,
    courses::CoursesController, enrollments::EnrollmentsController,
    shared::database::PostgresDatabase, users::UsersController,
};

use services::{
    mailer::{MailerConfig, MailerService},
    printer::DocumentPrinter,
    templates::TemplateConfig,
};

pub async fn init_test_app() -> TestServer {
    let app = Application::builder().expect("Failed to build Application");

    let pg_db_config = app.config.get::<PostgresDbConfig>().unwrap();
    let mailer_config = app.config.get::<MailerConfig>().unwrap();
    let template_config = app
        .config
        .get::<TemplateConfig>()
        .expect("Failed to get TemplateConfig");

    let postgres_db = PostgresDatabase::new(&pg_db_config)
        .await
        .expect("Failed to create database connection");

    postgres_db
        .migrate()
        .await
        .expect("Failed to migrate database");

    sqlx::query("TRUNCATE users, courses, enrollments, practices, reports CASCADE")
        .execute(&postgres_db.pool)
        .await
        .expect("Failed to truncate database tables");

    let smtp_transport = MailerService::new(&mailer_config, &template_config)
        .expect("Failed to create SMTP transport");

    let printer = DocumentPrinter::new(&template_config)
        .await
        .expect("Failed to create DocumentPrinter service");

    let dependency_container =
        DependencyContainer::new(postgres_db, smtp_transport, printer);

    let app = app
        .di_module(dependency_container.module)
        .expect("Failed to load DI module")
        .controller::<UsersController>()
        .controller::<CoursesController>()
        .controller::<EnrollmentsController>();

    TestServer::new(app.router()).expect("Failed to start test server")
}

pub fn extract_resource_id(data: &Value) -> String {
    data.get("id")
        .and_then(|id| id.as_str())
        .map(String::from)
        .unwrap_or_else(|| panic!("Response does not contain 'id': {data:?}"))
}
