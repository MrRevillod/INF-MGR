use bcrypt::hash;
use server::{
    courses::{Course, CourseEvaluation, CourseStatus},
    users::{Role, User},
};

use uuid::Uuid;

pub fn teachers() -> Vec<User> {
    vec![User {
        id: Uuid::new_v4(),
        rut: "21940032-2".to_string(),
        name: "Luis Alberto Caro Saldivia".to_string(),
        email: "lcaro@uct.cl".to_string(),
        roles: vec![Role::Teacher],
        created_at: chrono::Utc::now(),
        deleted_at: None,
        password: hash("!abc1234ABC.", 8).unwrap(),
    }]
}

pub fn coordinators() -> Vec<User> {
    vec![User {
        id: Uuid::new_v4(),
        rut: "38628573-k".to_string(),
        name: "Luis Diaz".to_string(),
        email: "ldiaz@inf.uct.cl".to_string(),
        roles: vec![Role::Coordinator],
        created_at: chrono::Utc::now(),
        deleted_at: None,
        password: hash("!abc1234ABC.", 8).unwrap(),
    }]
}

pub fn administrators() -> Vec<User> {
    vec![User {
        id: Uuid::new_v4(),
        rut: "41003110-8".to_string(),
        name: "Luciano Revillod".to_string(),
        email: "lrevillod2022@alu.uct.cl".to_string(),
        roles: vec![Role::Administrator],
        created_at: chrono::Utc::now(),
        deleted_at: None,
        password: hash("!abc1234ABC.", 8).unwrap(),
    }]
}

pub fn secretaries() -> Vec<User> {
    vec![User {
        id: Uuid::new_v4(),
        rut: "12345678-9".to_string(),
        name: "Secretaria".to_string(),
        email: "secretary@uct.cl".to_string(),
        roles: vec![Role::Secretary],
        created_at: chrono::Utc::now(),
        deleted_at: None,
        password: hash("!abc1234ABC.", 8).unwrap(),
    }]
}

pub fn evaluation_schemas() -> Vec<CourseEvaluation> {
    vec![
        CourseEvaluation {
            id: Uuid::new_v4(),
            name: "Informe de práctica".to_string(),
            weight: 30,
        },
        CourseEvaluation {
            id: Uuid::new_v4(),
            name: "Nota de Bitácoras".to_string(),
            weight: 30,
        },
        CourseEvaluation {
            id: Uuid::new_v4(),
            name: "Nota del supervisor de práctica".to_string(),
            weight: 40,
        },
    ]
}

pub fn info_1164(teachers: &[User], coordinators: &[User]) -> Course {
    let evaluation_schema = evaluation_schemas();

    Course {
        id: Uuid::new_v4(),
        name: "Práctica Inicial".to_string(),
        code: "INFO1164".to_string(),
        year: 2025,
        teacher_id: teachers.first().unwrap().id,
        coordinator_id: coordinators.first().unwrap().id,
        evaluations: evaluation_schema.clone(),
        status: CourseStatus::InProgress,
    }
}

pub fn info_1198(teachers: &[User], coordinators: &[User]) -> Course {
    let evaluation_schema = evaluation_schemas();

    Course {
        id: Uuid::new_v4(),
        name: "Práctica Profesional".to_string(),
        code: "INFO1198".to_string(),
        year: 2025,
        teacher_id: teachers.first().unwrap().id,
        coordinator_id: coordinators.first().unwrap().id,
        evaluations: evaluation_schema,
        status: CourseStatus::InProgress,
    }
}
