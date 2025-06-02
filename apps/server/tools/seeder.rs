#![cfg(feature = "seeder")]

use fake::faker::name::en::Name;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::task::JoinSet;

use sqlx::types::Json;

use chrono::Utc;
use fake::faker::internet::en::{FreeEmail, Password};
use fake::Fake;
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum UserRole {
    Student,
    Administrator,
    Coordinator,
    Mentor,
    Secretary,
}

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
    sqlx::query("TRUNCATE TABLE thesis_ideas, users RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await?;

    let mut tasks = JoinSet::new();

    for _ in 0..50 {
        let pool = pool.clone();
        tasks.spawn(async move {
            let now = Utc::now();
            let _ = sqlx::query(
                r#"
                INSERT INTO users (
                    id, name, email, password, validated, roles, created_at, updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                "#,
            )
            .bind(generate_random_rut())
            .bind(Name().fake::<String>())
            .bind(FreeEmail().fake::<String>())
            .bind(Password(8..12).fake::<String>())
            .bind(false)
            .bind(Json(vec![UserRole::Student]))
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

fn generate_random_rut() -> String {
    let mut rng = rand::rng();
    let number: u32 = rng.random_range(1..=99_999_999);

    let verifier = calculate_dv(number);

    format!("{}-{}", number, verifier)
}

fn calculate_dv(mut number: u32) -> String {
    let mut multiplier = 2;
    let mut sum = 0;

    while number > 0 {
        let digit = number % 10;
        sum += digit * multiplier;
        multiplier = if multiplier == 7 { 2 } else { multiplier + 1 };
        number /= 10;
    }

    let remainder = 11 - (sum % 11);
    match remainder {
        11 => "0".to_string(),
        10 => "K".to_string(),
        _ => remainder.to_string(),
    }
}
