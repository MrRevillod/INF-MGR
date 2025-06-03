use std::collections::HashSet;
use std::str::FromStr;

use axum::Router;
use axum_responses::{http::HttpResponse, response};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::config::{Config, CorsConfig, ServerConfig};
use crate::features::user::infrastructure::user_router;

use crate::shared::infrastructure::DependencyContainer;
use crate::shared::infrastructure::{logger::HttpLogger, PostgresDatabase};

pub struct Application {
    router: Router,
    config: ServerConfig,
}

impl Application {
    pub async fn new() -> Self {
        let config = Config::new().unwrap();

        dbg!(&config);

        let postgres_db = PostgresDatabase::new(config.database.postgres)
            .await
            .expect("Failed to create database connection");

        PostgresDatabase::migrate(&postgres_db.pool)
            .await
            .expect("Failed to create database connection");

        let di_container = DependencyContainer::new(postgres_db);
        let http_logger = HttpLogger::new();

        let cors_layer = Application::setup_cors(config.cors);

        let app_router = Router::new()
            .merge(user_router(di_container))
            .route("/health", axum::routing::get(Application::health_check))
            .layer(cors_layer)
            .layer(http_logger.layer);

        Application {
            router: app_router,
            config: config.server,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = TcpListener::bind(addr).await?;

        println!("Server listening on port 8000");

        axum::serve(listener, self.router.clone()).await?;

        Ok(())
    }

    pub async fn health_check() -> HttpResponse {
        let time = chrono::Utc::now().to_string();
        let status = "running";

        response!(200, { "status": status, "time": time })
    }

    pub fn setup_cors(config: CorsConfig) -> CorsLayer {
        let mut methods = HashSet::new();
        let mut headers = HashSet::new();

        for method in config.allowed_http_methods.iter() {
            let http_method = axum::http::Method::from_str(method)
                .expect("Invalid HTTP Method found in config");

            methods.insert(http_method);
        }

        for header in config.allowed_http_headers.iter() {
            let http_header = axum::http::header::HeaderName::from_str(header)
                .expect("Invalid HTTP Header found in config");

            headers.insert(http_header);
        }

        let methods = methods.into_iter().collect::<Vec<_>>();
        let headers = headers.into_iter().collect::<Vec<_>>();

        CorsLayer::new()
            .allow_methods(methods)
            .allow_headers(headers)
    }
}
