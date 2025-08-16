#![cfg(feature = "seeder")]

mod data;
mod functions;

use data::*;
use functions::*;

use bcrypt::hash;
use rand::Rng;

use server::users::{Role, User};

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use uuid::Uuid;

use fake::faker::{internet::en::FreeEmail, name::en::Name};
use fake::Fake;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_uri = std::env::var("POSTGRES_DATABASE_URL").expect("ENV POSTGRES_DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_uri)
        .await?;

    sqlx::query("TRUNCATE TABLE users, courses, enrollments, reports, practices CASCADE")
        .execute(&pool)
        .await?;

    sqlx::migrate!("./config/migrations").run(&pool).await?;

    let teachers = teachers();
    let administrators = administrators();
    let secretaries = secretaries();

    create_users(&pool, teachers.clone()).await;
    create_users(&pool, administrators.clone()).await;
    create_users(&pool, secretaries.clone()).await;

    let mut students = vec![];
    let info_1164_course = info_1164(&teachers);

    create_course(&pool, info_1164_course.clone()).await;

    for _ in 0..=10 {
        let student = User {
            id: Uuid::new_v4(),
            rut: generate_random_rut(),
            name: Name().fake(),
            email: FreeEmail().fake(),
            roles: vec![Role::Student],
            created_at: chrono::Utc::now(),
            deleted_at: None,
            password: hash("!abc1234ABC.", 8).unwrap(),
        };

        students.push(student);
    }

    create_users(&pool, students.clone()).await;
    create_enrollments(&pool, students, info_1164_course).await;

    let mut students2 = vec![];
    let info_1198_course = info_1198(&teachers);

    create_course(&pool, info_1198_course.clone()).await;

    for _ in 0..=10 {
        let student = User {
            id: Uuid::new_v4(),
            rut: generate_random_rut(),
            name: Name().fake(),
            email: FreeEmail().fake(),
            roles: vec![Role::Student],
            created_at: chrono::Utc::now(),
            deleted_at: None,
            password: hash("!abc1234ABC.", 8).unwrap(),
        };

        students2.push(student);
    }

    create_users(&pool, students2.clone()).await;
    create_enrollments(&pool, students2, info_1198_course).await;

    println!("Database seeded successfully!");

    Ok(())
}

fn generate_random_rut() -> String {
    let mut rng = rand::rng();
    let number: u32 = rng.random_range(1..=99_999_999);

    let verifier = calculate_dv(number);

    format!("{number}-{verifier}")
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
