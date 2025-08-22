use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{TEST_EMAILS, courses::utils::TestCourse, init_test_app, users::utils::TestUser};

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
    let mut students = Vec::new();

    for i in 0..10 {
        students.push(json!({
            "rut": TestUser::generate_unique_rut(),
            "name": format!("Student {}", i),
            "email": TestUser::generate_unique_email(),
        }));
    }

    students.push(json!({
        "rut": TestUser::generate_unique_rut(),
        "name": "Luciano Revillod",
        "email": TEST_EMAILS.get("student").unwrap_or(&TestUser::generate_unique_email())
    }));

    let teacher_id = TestUser::create_teacher(&app).await;

    let course_data = TestCourse::builder(&teacher_id).build();
    let course = TestCourse::create(&app, &course_data).await;
    let course_id = TestCourse::extract_id(&course);

    let course_import_data = json!({
        "id": course_id,
        "students": students
    });

    let response = app.post("/imports/course").json(&course_import_data).await;

    assert_eq!(response.status_code(), 201);

    cleanup_import_test_data(&course_id, &teacher_id).await;
}
