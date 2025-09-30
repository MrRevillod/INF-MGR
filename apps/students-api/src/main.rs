mod http;
mod middleware;
mod models;

pub use middleware::RequireApiKey;

use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::{sync::LazyLock, time::Duration};
use sword::prelude::*;

use crate::http::AppController;

#[derive(Serialize, Deserialize)]
#[config(key = "api")]
struct ApiConfig {
    db_url: String,
    api_key: String,
}

static COURSE_CODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^INFO\d{4}$").unwrap());

#[sword::main]
async fn main() {
    let app = Application::builder()?;

    let app_config = app.config.clone();
    let api_config = app_config.get::<ApiConfig>()?;

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .idle_timeout(Some(Duration::from_secs(300)))
        .connect(&api_config.db_url)
        .await?;

    let application = app
        .with_state(db_pool)?
        .with_controller::<AppController>()
        .build();

    application.run().await?;
}
