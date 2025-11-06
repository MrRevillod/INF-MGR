use server::{courses::Course, practices::Practice, users::User};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_users(pool: &Pool<Postgres>, users: Vec<User>) {
    let query = r"
            INSERT INTO users (id, rut, name, email, role, google_id, created_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ";

    for user in users {
        sqlx::query(query)
            .bind(user.id)
            .bind(&user.rut)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.role)
            .bind(&user.google_id)
            .bind(user.created_at)
            .bind(user.deleted_at)
            .execute(pool)
            .await
            .unwrap();
    }
}

pub async fn create_course(pool: &Pool<Postgres>, course: Course) {
    let query = r"
        INSERT INTO courses (id, name, code, year, teacher_id, evaluations, course_status)
        VALUES ($1, $2, $3, $4, $5, $6::course_evaluation[], $7::course_status)
    ";

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

pub async fn create_practices(pool: &Pool<Postgres>, practices: Vec<Practice>) {
    let query = r"
        INSERT INTO practices (id, enterprise_name, location, description, supervisor_name, supervisor_email, supervisor_phone, start_date, end_date, practice_status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::practice_status)
    ";

    for practice in practices {
        sqlx::query(query)
            .bind(practice.id)
            .bind(&practice.enterprise_name)
            .bind(&practice.location)
            .bind(&practice.description)
            .bind(&practice.supervisor_name)
            .bind(&practice.supervisor_email)
            .bind(&practice.supervisor_phone)
            .bind(practice.start_date)
            .bind(practice.end_date)
            .bind(practice.practice_status)
            .execute(pool)
            .await
            .unwrap();
    }
}

pub async fn create_enrollments(
    pool: &Pool<Postgres>,
    students: Vec<User>,
    course: Course,
    practices: Vec<Practice>,
) {
    let query = r"
        INSERT INTO enrollments (id, student_id, course_id, practice_id, student_scores)
        VALUES ($1, $2, $3, $4, ARRAY[]::student_score[])
    ";

    for (student, practice) in students.into_iter().zip(practices) {
        sqlx::query(query)
            .bind(Uuid::new_v4())
            .bind(student.id)
            .bind(course.id)
            .bind(practice.id)
            .execute(pool)
            .await
            .unwrap();
    }
}
