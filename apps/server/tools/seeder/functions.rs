use server::{courses::Course, users::User};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_users(pool: &Pool<Postgres>, users: Vec<User>) {
    let query = r#"
            INSERT INTO users (id, rut, name, email, roles, password, created_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5::user_role[], $6, $7, $8)
        "#;

    for user in users {
        sqlx::query(query)
            .bind(user.id)
            .bind(&user.rut)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.roles)
            .bind(&user.password)
            .bind(user.created_at)
            .bind(user.deleted_at)
            .execute(pool)
            .await
            .unwrap();
    }
}

pub async fn create_course(pool: &Pool<Postgres>, course: Course) {
    let query = r#"
        INSERT INTO courses (id, name, code, year, teacher_id, evaluations, course_status)
        VALUES ($1, $2, $3, $4, $5, $6, $7::course_evaluation[], $8::course_status)
    "#;

    sqlx::query(query)
        .bind(course.id)
        .bind(&course.name)
        .bind(&course.code)
        .bind(course.year)
        .bind(course.teacher_id)
        .bind(&course.evaluations)
        .bind(course.course_status)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn create_enrollments(
    pool: &Pool<Postgres>,
    students: Vec<User>,
    course: Course,
) {
    let query = r#"
        INSERT INTO enrollments (id, student_id, course_id, practice_id, student_scores)
        VALUES ($1, $2, $3, NULL, ARRAY[]::student_score[])
    "#;

    for student in students {
        sqlx::query(query)
            .bind(Uuid::new_v4())
            .bind(student.id)
            .bind(course.id)
            .execute(pool)
            .await
            .unwrap();
    }
}
