#![cfg(feature = "seeder")]

mod data;
mod functions;

use data::*;
use functions::*;

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

    sqlx::query("TRUNCATE TABLE users, courses, enrollments, practices, meeting_requests, meetings CASCADE")
        .execute(&pool)
        .await?;

    sqlx::migrate!("./config/migrations").run(&pool).await?;

    let teachers = teachers();
    let students = students();

    create_users(&pool, teachers.clone()).await;
    create_users(&pool, students.clone()).await;
    create_users(&pool, administrators()).await;

    let info_1164_course = info_1164(&teachers);

    create_course(&pool, info_1164_course.clone()).await;

    let practices = practices(students.len());
    create_practices(&pool, practices.clone()).await;

    create_enrollments(&pool, students, info_1164_course, practices).await;

    println!("Database seeded successfully!");

    Ok(())
}
