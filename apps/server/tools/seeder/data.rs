use server::{
    courses::{Course, CourseEvaluation, CourseStatus},
    users::{Role, User},
};

use chrono::Utc;
use uuid::Uuid;

pub fn students() -> Vec<User> {
    vec![
        // Usuario de prueba desde variable de entorno
        User {
            id: Uuid::new_v4(),
            rut: "56315776-3".to_string(),
            name: "Student USER".to_string(),
            email: std::env::var("TEST_STUDENT_EMAIL")
                .expect("TEST_STUDENT_EMAIL must be set in .env"),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        // Estudiantes adicionales para pruebas
        User {
            id: Uuid::new_v4(),
            rut: "16111111-1".to_string(),
            name: "Juan Pérez".to_string(),
            email: "juan.perez@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "17222222-2".to_string(),
            name: "María Silva".to_string(),
            email: "maria.silva@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "18333333-3".to_string(),
            name: "Pedro González".to_string(),
            email: "pedro.gonzalez@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "19444444-4".to_string(),
            name: "Lucía Morales".to_string(),
            email: "lucia.morales@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "20555555-5".to_string(),
            name: "Diego Castro".to_string(),
            email: "diego.castro@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "21666666-6".to_string(),
            name: "Sofía Ramírez".to_string(),
            email: "sofia.ramirez@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "22777777-7".to_string(),
            name: "Andrés Torres".to_string(),
            email: "andres.torres@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "23888888-8".to_string(),
            name: "Camila Vargas".to_string(),
            email: "camila.vargas@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "24999999-9".to_string(),
            name: "Felipe Muñoz".to_string(),
            email: "felipe.munoz@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "25000000-0".to_string(),
            name: "Valentina Rojas".to_string(),
            email: "valentina.rojas@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "26111111-1".to_string(),
            name: "Nicolás Herrera".to_string(),
            email: "nicolas.herrera@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "27222222-2".to_string(),
            name: "Francisca Ponce".to_string(),
            email: "francisca.ponce@estudiante.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
    ]
}

pub fn teachers() -> Vec<User> {
    vec![
        // Usuario de prueba desde variable de entorno
        User {
            id: Uuid::new_v4(),
            rut: "21940032-2".to_string(),
            name: "Teacher USER".to_string(),
            email: std::env::var("TEST_TEACHER_EMAIL")
                .expect("TEST_TEACHER_EMAIL must be set in .env"),
            role: Role::Teacher,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        // Profesores adicionales para pruebas
        User {
            id: Uuid::new_v4(),
            rut: "11222222-1".to_string(),
            name: "Dr. Carlos García".to_string(),
            email: "prof.garcia@universidad.cl".to_string(),
            role: Role::Teacher,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "11222222-2".to_string(),
            name: "Dra. Ana López".to_string(),
            email: "prof.lopez@universidad.cl".to_string(),
            role: Role::Teacher,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "12333333-3".to_string(),
            name: "Dr. Pedro Martínez".to_string(),
            email: "prof.martinez@universidad.cl".to_string(),
            role: Role::Teacher,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "13444444-4".to_string(),
            name: "Dra. Laura Rodríguez".to_string(),
            email: "prof.rodriguez@universidad.cl".to_string(),
            role: Role::Teacher,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
    ]
}

pub fn administrators() -> Vec<User> {
    vec![
        // Usuario de prueba desde variable de entorno
        User {
            id: Uuid::new_v4(),
            rut: "11111111-1".to_string(),
            name: "Admin USER".to_string(),
            email: std::env::var("TEST_ADMIN_EMAIL")
                .expect("TEST_ADMIN_EMAIL must be set in .env"),
            role: Role::Administrator,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        // Administradores adicionales para pruebas
        User {
            id: Uuid::new_v4(),
            rut: "12345678-9".to_string(),
            name: "Admin Principal".to_string(),
            email: "admin@universidad.cl".to_string(),
            role: Role::Administrator,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "12345678-9".to_string(),
            name: "María González".to_string(),
            email: "admin2@universidad.cl".to_string(),
            role: Role::Administrator,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
    ]
}

// pub fn evaluation_schemas() -> Vec<CourseEvaluation> {
//     vec![
//         CourseEvaluation {
//             id: Uuid::new_v4(),
//             name: "Informe de práctica".to_string(),
//             weight: 60,
//         },
//         CourseEvaluation {
//             id: Uuid::new_v4(),
//             name: "Nota del supervisor de práctica".to_string(),
//             weight: 40,
//         },
//     ]
// }

// pub fn info_1164(teachers: &[User]) -> Course {
//     let evaluation_schema = evaluation_schemas();

//     Course {
//         id: Uuid::new_v4(),
//         name: "Práctica Inicial".to_string(),
//         code: "INFO1164".to_string(),
//         year: 2025,
//         teacher_id: teachers.first().unwrap().id,
//         evaluations: evaluation_schema,
//         course_status: CourseStatus::Active,
//     }
// }

// Cursos de prácticas
pub fn additional_courses(teachers: &[User]) -> Vec<Course> {
    vec![
        Course {
            id: Uuid::new_v4(),
            name: "Práctica Inicial".to_string(),
            code: "INFO1164".to_string(),
            year: 2024,
            teacher_id: teachers.get(0).unwrap().id,
            evaluations: vec![
                CourseEvaluation {
                    id: Uuid::new_v4(),
                    name: "Informe de Práctica".to_string(),
                    weight: 40,
                },
                CourseEvaluation {
                    id: Uuid::new_v4(),
                    name: "Evaluación del Supervisor".to_string(),
                    weight: 30,
                },
                CourseEvaluation {
                    id: Uuid::new_v4(),
                    name: "Presentación Final".to_string(),
                    weight: 30,
                },
            ],
            course_status: CourseStatus::Active,
        },
        Course {
            id: Uuid::new_v4(),
            name: "Práctica Profesional".to_string(),
            code: "INFO1264".to_string(),
            year: 2025,
            teacher_id: teachers.get(1).unwrap().id,
            evaluations: vec![
                CourseEvaluation {
                    id: Uuid::new_v4(),
                    name: "Informe de Práctica".to_string(),
                    weight: 40,
                },
                CourseEvaluation {
                    id: Uuid::new_v4(),
                    name: "Evaluación del Supervisor".to_string(),
                    weight: 30,
                },
                CourseEvaluation {
                    id: Uuid::new_v4(),
                    name: "Defensa de Práctica".to_string(),
                    weight: 30,
                },
            ],
            course_status: CourseStatus::Active,
        },
    ]
}
