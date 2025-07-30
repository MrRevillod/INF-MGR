#![cfg(feature = "seeder")]

mod asignatures;
mod users;

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

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
    users::seed_users_table_administrator(&pool).await?;
    users::seed_users_table_coordinator(&pool).await?;
    users::seed_users_table_secretary(&pool).await?;
    users::seed_users_table_teachers(&pool).await?;
    users::seed_users_table_students(&pool).await?;

    // Seed asignatures y luego inscripciones
    let asignature_ids = asignatures::seed_asignatures(&pool).await?;
    asignatures::seed_inscriptions(&pool, &asignature_ids).await?;

    println!("Database seeded successfully!");

    Ok(())
}
