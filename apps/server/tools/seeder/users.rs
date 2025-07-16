#![cfg(feature = "seeder")]

use fake::faker::name::en::Name;
use sqlx::PgPool;
use tokio::task::JoinSet;

use fake::faker::internet::en::{FreeEmail, Password};
use fake::Fake;
use rand::Rng;

pub async fn seed_users_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tasks = JoinSet::new();

    let query = r#"
        INSERT INTO users (id, rut, name, email, password, role) 
        VALUES ($1, $2, $3, $4, $5, $6)
    "#;

    for _ in 0..50 {
        let pool = pool.clone();
        tasks.spawn(async move {
            sqlx::query(query)
                .bind(uuid::Uuid::new_v4())
                .bind(generate_random_rut())
                .bind(Name().fake::<String>())
                .bind(FreeEmail().fake::<String>())
                .bind(Password(8..12).fake::<String>())
                .bind("student")
                .execute(&pool)
                .await
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
