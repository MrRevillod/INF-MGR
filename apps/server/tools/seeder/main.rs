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

    println!("🔗 Conectado a la base de datos");
    println!("🔌 Terminando otras conexiones...");

    sqlx::query(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity
         WHERE datname = current_database() AND pid <> pg_backend_pid()",
    )
    .execute(&pool)
    .await?;

    println!("🗑️  Eliminando tablas...");

    sqlx::query("DROP TABLE IF EXISTS enrollments, practices, courses, users CASCADE")
        .execute(&pool)
        .await?;

    sqlx::query("TRUNCATE TABLE _sqlx_migrations")
        .execute(&pool)
        .await?;

    println!("📦 Ejecutando migraciones...");
    sqlx::migrate!("./config/migrations").run(&pool).await?;

    let teachers = teachers();
    let students = students();

    create_users(&pool, teachers.clone()).await;
    create_users(&pool, students.clone()).await;
    create_users(&pool, administrators()).await;
    create_users(&pool, secretaries()).await;

    let extra_courses = additional_courses(&teachers);

    // Crear cursos y enrollments
    for course in extra_courses.iter() {
        create_course(&pool, course.clone()).await;
        let students_for_course: Vec<_> = students.iter().take(6).cloned().collect();
        create_enrollments(&pool, students_for_course, course.clone()).await;
    }

    // Crear prácticas y asociarlas a algunos enrollments
    let practices = sample_practices();

    // Asociar prácticas a diferentes estudiantes en diferentes cursos
    // Estudiante 0 - Curso 0 (INF-101)
    /* if let Some(student) = students.get(0) {
        if let Some(course) = extra_courses.get(0) {
            if let Some(practice) = practices.get(0) {
                let practice_id = create_practice(&pool, practice.clone()).await;
                if let Some(enrollment_id) =
                    get_enrollment_id(&pool, student.id, course.id).await
                {
                    update_enrollment_with_practice(&pool, enrollment_id, practice_id)
                        .await;
                    println!(
                        "✓ Práctica asignada a {} en curso {}",
                        student.name, course.name
                    );
                }
            }
        }
    } */

    /* // Estudiante 1 - Curso 1 (INF-201)
    if let Some(student) = students.get(1) {
        if let Some(course) = extra_courses.get(1) {
            if let Some(practice) = practices.get(1) {
                let practice_id = create_practice(&pool, practice.clone()).await;
                if let Some(enrollment_id) =
                    get_enrollment_id(&pool, student.id, course.id).await
                {
                    update_enrollment_with_practice(&pool, enrollment_id, practice_id)
                        .await;
                    println!(
                        "✓ Práctica asignada a {} en curso {}",
                        student.name, course.name
                    );
                }
            }
        }
    } */

    // Estudiante 2 - Curso 2 (INF-202)
    if let Some(student) = students.get(2) {
        if let Some(course) = extra_courses.get(2) {
            if let Some(practice) = practices.get(2) {
                let practice_id = create_practice(&pool, practice.clone()).await;
                if let Some(enrollment_id) =
                    get_enrollment_id(&pool, student.id, course.id).await
                {
                    update_enrollment_with_practice(&pool, enrollment_id, practice_id)
                        .await;
                    println!(
                        "✓ Práctica asignada a {} en curso {}",
                        student.name, course.name
                    );
                }
            }
        }
    }

    /* // Estudiante 3 - Curso 0 (INF-101)
    if let Some(student) = students.get(3) {
        if let Some(course) = extra_courses.get(0) {
            if let Some(practice) = practices.get(3) {
                let practice_id = create_practice(&pool, practice.clone()).await;
                if let Some(enrollment_id) =
                    get_enrollment_id(&pool, student.id, course.id).await
                {
                    update_enrollment_with_practice(&pool, enrollment_id, practice_id)
                        .await;
                    println!(
                        "✓ Práctica asignada a {} en curso {}",
                        student.name, course.name
                    );
                }
            }
        }
    } */

    // Estudiante 4 - Curso 1 (INF-201) - Práctica Pendiente
    /* if let Some(student) = students.get(4) {
        if let Some(course) = extra_courses.get(1) {
            if let Some(practice) = practices.get(4) {
                let practice_id = create_practice(&pool, practice.clone()).await;
                if let Some(enrollment_id) =
                    get_enrollment_id(&pool, student.id, course.id).await
                {
                    update_enrollment_with_practice(&pool, enrollment_id, practice_id)
                        .await;
                    println!(
                        "✓ Práctica asignada a {} en curso {}",
                        student.name, course.name
                    );
                }
            }
        }
    } */

    // Estudiante 5 - Curso 2 (INF-202)
    if let Some(student) = students.get(5) {
        if let Some(course) = extra_courses.get(2) {
            if let Some(practice) = practices.get(5) {
                let practice_id = create_practice(&pool, practice.clone()).await;
                if let Some(enrollment_id) =
                    get_enrollment_id(&pool, student.id, course.id).await
                {
                    update_enrollment_with_practice(&pool, enrollment_id, practice_id)
                        .await;
                    println!(
                        "✓ Práctica asignada a {} en curso {}",
                        student.name, course.name
                    );
                }
            }
        }
    }

    println!("\n🎉 Database seeded successfully!");
    println!("   - {} estudiantes creados", students.len());
    println!("   - {} profesores creados", teachers.len());
    println!("   - {} cursos creados", extra_courses.len());
    println!("   - 6 prácticas creadas y asignadas");

    Ok(())
}
