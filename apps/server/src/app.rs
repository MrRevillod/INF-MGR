use std::sync::Arc;

use axum::Router;
use axum_responses::{http::HttpResponse, response};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::features::user::infrastructure::user_router;

use crate::shared::infrastructure::{
    logger::HttpLogger,
    PostgresDatabase, {AppModule, AppState},
};

use crate::shared::constants::{
    check_env_vars, ALLOWED_HTTP_HEADERS, ALLOWED_HTTP_METHODS,
};

pub struct Application {
    router: Router,
}

impl Application {
    pub async fn new() -> Self {
        check_env_vars();

        let di_state = Application::set_up_di().await;

        let http_logger = HttpLogger::new();
        let cors_layer = CorsLayer::new()
            .allow_methods(ALLOWED_HTTP_METHODS.to_owned())
            .allow_headers(ALLOWED_HTTP_HEADERS.to_owned());

        let app_router = Router::new()
            .merge(user_router(di_state))
            .route("/health", axum::routing::get(Application::health_check))
            .layer(cors_layer)
            .layer(http_logger.layer);

        Application { router: app_router }
    }

    pub async fn set_up_di() -> AppState {
        let db_connection = PostgresDatabase::new()
            .await
            .expect("Failed to create database connection");

        db_connection
            .migrate()
            .await
            .expect("Failed to run database migrations");

        let di_module = AppModule::builder()
            .with_component_parameters::<PostgresDatabase>(db_connection.into())
            .build();

        AppState {
            module: Arc::new(di_module),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("0.0.0.0:8000").await?;

        println!("Server listening on port 8000");

        axum::serve(listener, self.router.clone()).await?;

        Ok(())
    }

    pub async fn health_check() -> HttpResponse {
        let time = chrono::Utc::now();
        let status = "running";

        response!(200, { "status": status, "time": time.to_string() })
    }
}
