use server::{
    courses::{Course, CourseEvaluation, CourseStatus},
    practices::{Practice, PracticeStatus},
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
            register: Some("123456789".to_string()),
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
            register: Some("987654321".to_string()),
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
            register: Some("132465798".to_string()),
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
            register: Some("564738291".to_string()),
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
            register: Some("192837465".to_string()),
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
            register: Some("847362915".to_string()),
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
            register: Some("564738192".to_string()),
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
            register: Some("918273645".to_string()),
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
            register: Some("374829165".to_string()),
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
            register: Some("456123789".to_string()),
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
            register: Some("678912345".to_string()),
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
            register: Some("123459876".to_string()),
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
            register: Some("475869132".to_string()),
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
            register: None,
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
            register: None,
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
            register: None,
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
            register: None,
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
            register: None,
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
            register: None,
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
            register: None,
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
            register: None,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
    ]
}

pub fn secretaries() -> Vec<User> {
    vec![
        // Usuario de prueba desde variable de entorno
        User {
            id: Uuid::new_v4(),
            rut: "10000000-0".to_string(),
            name: "Secretary USER".to_string(),
            email: std::env::var("TEST_SECRETARY_EMAIL")
                .expect("TEST_SECRETARY_EMAIL must be set in .env"),
            role: Role::Secretary,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
            register: None,
        },
        // Secretarias adicionales para pruebas
        User {
            id: Uuid::new_v4(),
            rut: "10111111-1".to_string(),
            name: "Carolina Fuentes".to_string(),
            email: "secretary.fuentes@universidad.cl".to_string(),
            role: Role::Secretary,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
            register: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "10222222-2".to_string(),
            name: "Patricia Soto".to_string(),
            email: "secretary.soto@universidad.cl".to_string(),
            role: Role::Secretary,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
            register: None,
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

// Prácticas de ejemplo
pub fn sample_practices() -> Vec<Practice> {
    use chrono::Duration;
    let now = Utc::now();

    vec![
        Practice {
            id: Uuid::new_v4(),
            enterprise_name: "TechSolutions Chile".to_string(),
            location: "Santiago, Chile".to_string(),
            description: "Desarrollo de aplicaciones web con React y Node.js".to_string(),
            supervisor_name: "Roberto Sánchez".to_string(),
            supervisor_email: "r.sanchez@techsolutions.cl".to_string(),
            supervisor_phone: "+56912345678".to_string(),
            start_date: Some(now - Duration::days(30)),
            end_date: Some(now + Duration::days(60)),
            practice_status: PracticeStatus::Approved,
        },
        Practice {
            id: Uuid::new_v4(),
            enterprise_name: "DataCorp".to_string(),
            location: "Valparaíso, Chile".to_string(),
            description: "Análisis de datos y desarrollo de modelos de machine learning"
                .to_string(),
            supervisor_name: "Carmen López".to_string(),
            supervisor_email: "carmen.lopez@datacorp.cl".to_string(),
            supervisor_phone: "+56987654321".to_string(),
            start_date: Some(now - Duration::days(15)),
            end_date: Some(now + Duration::days(75)),
            practice_status: PracticeStatus::Approved,
        },
        Practice {
            id: Uuid::new_v4(),
            enterprise_name: "InnovaSoft".to_string(),
            location: "Concepción, Chile".to_string(),
            description: "Desarrollo de aplicaciones móviles con Flutter".to_string(),
            supervisor_name: "Jorge Morales".to_string(),
            supervisor_email: "j.morales@innovasoft.cl".to_string(),
            supervisor_phone: "+56923456789".to_string(),
            start_date: Some(now - Duration::days(45)),
            end_date: Some(now + Duration::days(45)),
            practice_status: PracticeStatus::Approved,
        },
        Practice {
            id: Uuid::new_v4(),
            enterprise_name: "CloudServices SpA".to_string(),
            location: "Santiago, Chile".to_string(),
            description: "Administración de infraestructura cloud en AWS".to_string(),
            supervisor_name: "Patricia Díaz".to_string(),
            supervisor_email: "p.diaz@cloudservices.cl".to_string(),
            supervisor_phone: "+56934567890".to_string(),
            start_date: Some(now - Duration::days(10)),
            end_date: Some(now + Duration::days(80)),
            practice_status: PracticeStatus::Approved,
        },
        Practice {
            id: Uuid::new_v4(),
            enterprise_name: "CyberSec Chile".to_string(),
            location: "Viña del Mar, Chile".to_string(),
            description: "Análisis de seguridad informática y pentesting".to_string(),
            supervisor_name: "Miguel Fernández".to_string(),
            supervisor_email: "m.fernandez@cybersec.cl".to_string(),
            supervisor_phone: "+56945678901".to_string(),
            start_date: None,
            end_date: None,
            practice_status: PracticeStatus::Pending,
        },
        Practice {
            id: Uuid::new_v4(),
            enterprise_name: "GameDev Studios".to_string(),
            location: "Santiago, Chile".to_string(),
            description: "Desarrollo de videojuegos con Unity".to_string(),
            supervisor_name: "Andrea Ruiz".to_string(),
            supervisor_email: "a.ruiz@gamedev.cl".to_string(),
            supervisor_phone: "+56956789012".to_string(),
            start_date: Some(now - Duration::days(20)),
            end_date: Some(now + Duration::days(70)),
            practice_status: PracticeStatus::Approved,
        },
    ]
}
