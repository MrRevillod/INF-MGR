#![cfg(feature = "seeder")]

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::task::JoinSet;

use chrono::Utc;
use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::Fake;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_uri = std::env::var("POSTGRES_DATABASE_URL")
        .expect("ENV POSTGRES_DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_uri)
        .await?;

    sqlx::migrate!("./config/migrations").run(&pool).await?;
    sqlx::query("TRUNCATE TABLE users").execute(&pool).await?;

    let mut tasks = JoinSet::new();

    for _ in 0..50 {
        let pool = pool.clone();
        tasks.spawn(async move {
            let now = Utc::now();
            let _ = sqlx::query(
                r#"
                INSERT INTO users (
                    id, username, email, password, validated, created_at, updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(Username().fake::<String>())
            .bind(FreeEmail().fake::<String>())
            .bind(Password(8..12).fake::<String>())
            .bind(false)
            .bind(now)
            .bind(now)
            .execute(&pool)
            .await;
        });
    }

    while let Some(res) = tasks.join_next().await {
        if let Err(e) = res {
            eprintln!("Error en tarea: {e}");
        }
    }

    Ok(())
}
