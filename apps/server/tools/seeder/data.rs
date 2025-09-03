use server::{
    courses::{Course, CourseEvaluation, CourseStatus},
    users::{Role, User},
};

use uuid::Uuid;

pub fn students() -> Vec<User> {
    vec![User {
        id: Uuid::new_v4(),
        rut: "56315776-3".to_string(),
        name: "Student USER".to_string(),
        email: std::env::var("TEST_STUDENT_EMAIL").expect("TEST_STUDENT_EMAIL must be set in .env"),
        roles: vec![Role::Student],
        created_at: chrono::Utc::now(),
        deleted_at: None,
        google_id: None,
    }]
}

pub fn teachers() -> Vec<User> {
    vec![User {
        id: Uuid::new_v4(),
        rut: "21940032-2".to_string(),
        name: "Teacher USER".to_string(),
        email: std::env::var("TEST_TEACHER_EMAIL").expect("TEST_TEACHER_EMAIL must be set in .env"),
        roles: vec![Role::Teacher],
        created_at: chrono::Utc::now(),
        deleted_at: None,
        google_id: None,
    }]
}

pub fn evaluation_schemas() -> Vec<CourseEvaluation> {
    vec![
        CourseEvaluation {
            id: Uuid::new_v4(),
            name: "Informe de práctica".to_string(),
            weight: 60,
        },
        CourseEvaluation {
            id: Uuid::new_v4(),
            name: "Nota del supervisor de práctica".to_string(),
            weight: 40,
        },
    ]
}

pub fn info_1164(teachers: &[User]) -> Course {
    let evaluation_schema = evaluation_schemas();

    Course {
        id: Uuid::new_v4(),
        name: "Práctica Inicial".to_string(),
        code: "INFO1164".to_string(),
        year: 2025,
        teacher_id: teachers.first().unwrap().id,
        evaluations: evaluation_schema.clone(),
        course_status: CourseStatus::Active,
    }
}
