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

    let extra_courses = additional_courses(&teachers);

    for course in extra_courses.iter() {
        create_course(&pool, course.clone()).await;
        let students_for_course: Vec<_> = students.iter().take(6).cloned().collect();
        create_enrollments(&pool, students_for_course, course.clone()).await;
    }

    println!("Database seeded successfully!");

    Ok(())
}
