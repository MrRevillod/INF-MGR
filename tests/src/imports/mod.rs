use std::env::var;

use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    courses::utils::{CourseBuilder, create_course},
    extract_resource_id, init_test_app,
    users::utils::{create_teacher, generate_unique_email, generate_unique_rut},
};

async fn get_db_pool() -> PgPool {
    use server::{config::PostgresDbConfig, shared::database::PostgresDatabase};
    use sword::prelude::Application;

    let app = Application::builder().expect("Failed to create application builder");
    let pg_db_config =
        app.config.get::<PostgresDbConfig>().expect("Failed to get PostgresDbConfig");

    let db = PostgresDatabase::new(&pg_db_config)
        .await
        .expect("Failed to create database connection");

    db.pool
}

async fn cleanup_import_test_data(course_id: &str, teacher_id: &str) {
    let pool = get_db_pool().await;
    let course_uuid = Uuid::parse_str(course_id).unwrap();
    let teacher_uuid = Uuid::parse_str(teacher_id).unwrap();

    sqlx::query("DELETE FROM enrollments WHERE course_id = $1")
        .bind(course_uuid)
        .execute(&pool)
        .await
        .expect("Failed to delete enrollments");

    let student_ids: Vec<Uuid> = sqlx::query_scalar(
        "SELECT DISTINCT u.id FROM users u 
         WHERE u.roles = ARRAY['student']::user_role[] 
         AND u.created_at > NOW() - INTERVAL '1 minute'",
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to get student IDs");

    for student_id in student_ids {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(student_id)
            .execute(&pool)
            .await
            .expect("Failed to delete student");
    }

    sqlx::query("DELETE FROM courses WHERE id = $1")
        .bind(course_uuid)
        .execute(&pool)
        .await
        .expect("Failed to delete course");

    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(teacher_uuid)
        .execute(&pool)
        .await
        .expect("Failed to delete teacher");
}

#[tokio::test]
async fn test_import_users() {
    let app = init_test_app().await;

    let student_email = var("STUDENT_EMAIL").expect("STUDENT_EMAIL not set");
    let teacher_email = var("TEACHER_EMAIL").expect("TEACHER_EMAIL not set");

    let mut students = Vec::new();

    for i in 0..10 {
        students.push(json!({
            "rut": generate_unique_rut(),
            "name": format!("Student {}", i),
            "email": generate_unique_email(),
        }));
    }

    students.push(json!({
        "rut": generate_unique_rut(),
        "name": "Luciano Revillod",
        "email": student_email,
    }));

    let teacher_id = create_teacher(&app, Some(teacher_email)).await;

    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;

    let course_id = extract_resource_id(&course);

    let course_import_data = json!({
        "id": course_id,
        "students": students
    });

    let response = app.post("/imports/course").json(&course_import_data).await;

    assert_eq!(response.status_code(), 201);

    cleanup_import_test_data(&course_id, &teacher_id).await;
}
