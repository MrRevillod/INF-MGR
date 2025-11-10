use server::{
    courses::{Course, CourseEvaluation, CourseStatus},
    practices::{Practice, PracticeStatus},
    users::{Role, User},
};

use chrono::Utc;
use uuid::Uuid;

pub fn students() -> Vec<User> {
    vec![
        // Usuario original con variable de entorno para testing
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
        User {
            id: Uuid::new_v4(),
            rut: "20123456-7".to_string(),
            name: "María José González".to_string(),
            email: "maria.gonzalez@estudiante.uach.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "19876543-2".to_string(),
            name: "Carlos Eduardo Ramírez".to_string(),
            email: "carlos.ramirez@estudiante.uach.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "21345678-9".to_string(),
            name: "Ana Sofía Mendoza".to_string(),
            email: "ana.mendoza@estudiante.uach.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "20567890-1".to_string(),
            name: "Diego Alejandro Torres".to_string(),
            email: "diego.torres@estudiante.uach.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "19234567-8".to_string(),
            name: "Valentina Isabel Morales".to_string(),
            email: "valentina.morales@estudiante.uach.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "21678901-3".to_string(),
            name: "Sebastián Andrés López".to_string(),
            email: "sebastian.lopez@estudiante.uach.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "20890123-4".to_string(),
            name: "Francisca Nicole Rivera".to_string(),
            email: "francisca.rivera@estudiante.uach.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "19456789-0".to_string(),
            name: "Matías Ignacio Hernández".to_string(),
            email: "matias.hernandez@estudiante.uach.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "21012345-6".to_string(),
            name: "Camila Fernanda Silva".to_string(),
            email: "camila.silva@estudiante.uach.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
        User {
            id: Uuid::new_v4(),
            rut: "20234567-K".to_string(),
            name: "Benjamín Esteban Castillo".to_string(),
            email: "benjamin.castillo@estudiante.uach.cl".to_string(),
            role: Role::Student,
            created_at: Utc::now(),
            deleted_at: None,
            google_id: None,
        },
    ]
}

pub fn teachers() -> Vec<User> {
    vec![User {
        id: Uuid::new_v4(),
        rut: "21940032-2".to_string(),
        name: "Teacher USER".to_string(),
        email: std::env::var("TEST_TEACHER_EMAIL")
            .expect("TEST_TEACHER_EMAIL must be set in .env"),
        role: Role::Teacher,
        created_at: Utc::now(),
        deleted_at: None,
        google_id: None,
    }]
}

pub fn administrators() -> Vec<User> {
    vec![User {
        id: Uuid::new_v4(),
        rut: "11111111-1".to_string(),
        name: "Admin USER".to_string(),
        email: std::env::var("TEST_ADMIN_EMAIL")
            .expect("TEST_ADMIN_EMAIL must be set in .env"),
        role: Role::Administrator,
        created_at: Utc::now(),
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
        evaluations: evaluation_schema,
        course_status: CourseStatus::Active,
    }
}

pub fn practices(count: usize) -> Vec<Practice> {
    let practice_data = vec![
        (
            "Banco de Chile",
            "Santiago, Región Metropolitana",
            "Desarrollo de aplicaciones móviles bancarias utilizando tecnologías modernas como React Native y APIs REST para mejorar la experiencia del usuario en servicios financieros digitales.",
            "Patricia Sánchez",
            "patricia.sanchez@bancochile.cl",
            "+56 2 2653 1000",
        ),
        (
            "Falabella.com",
            "Las Condes, Santiago",
            "Implementación de soluciones e-commerce y desarrollo de microservicios para la plataforma de ventas online, trabajando con tecnologías cloud y metodologías ágiles.",
            "Roberto Martínez",
            "roberto.martinez@falabella.com",
            "+56 2 2379 9000",
        ),
        (
            "SONDA S.A.",
            "Providencia, Santiago",
            "Desarrollo de sistemas de gestión empresarial (ERP) y consultoría en transformación digital para clientes del sector público y privado.",
            "Andrea Jiménez",
            "andrea.jimenez@sonda.com",
            "+56 2 2657 5000",
        ),
        (
            "Everis Chile",
            "Vitacura, Santiago",
            "Desarrollo de aplicaciones web con Angular y Spring Boot, participación en proyectos de digitalización para empresas del sector financiero y retail.",
            "Fernando Rojas",
            "fernando.rojas@everis.com",
            "+56 2 2424 3500",
        ),
        (
            "Telefónica Chile",
            "Las Condes, Santiago",
            "Desarrollo de soluciones de telecomunicaciones y sistemas de gestión de redes, trabajando con tecnologías 5G y IoT para mejorar la conectividad nacional.",
            "Claudia Vega",
            "claudia.vega@telefonica.cl",
            "+56 2 2691 9000",
        ),
        (
            "Nisum Technologies",
            "Providencia, Santiago",
            "Desarrollo de software para clientes internacionales, especialización en arquitecturas de microservicios y desarrollo full-stack con tecnologías JavaScript modernas.",
            "Miguel Contreras",
            "miguel.contreras@nisum.com",
            "+56 2 2233 4400",
        ),
        (
            "Banco Santander Chile",
            "Santiago Centro",
            "Desarrollo de plataforma de banca digital y sistemas de análisis de datos para la toma de decisiones financieras, utilizando Python y tecnologías de Big Data.",
            "Lorena Moreno",
            "lorena.moreno@santander.cl",
            "+56 2 2320 8000",
        ),
        (
            "Walmart Chile",
            "Quilicura, Santiago",
            "Desarrollo de sistemas de gestión de inventario y logística, implementación de soluciones de automatización para cadena de suministro.",
            "Carlos Herrera",
            "carlos.herrera@walmart.cl",
            "+56 2 2587 7000",
        ),
        (
            "Cornershop by Uber",
            "Las Condes, Santiago",
            "Desarrollo de aplicaciones móviles para delivery y gestión de pedidos, trabajando con APIs de geolocalización y sistemas de notificaciones push.",
            "Sofía Paredes",
            "sofia.paredes@cornershopapp.com",
            "+56 2 2897 4500",
        ),
        (
            "Transbank S.A.",
            "Las Condes, Santiago",
            "Desarrollo de sistemas de pagos electrónicos y procesamiento de transacciones financieras, implementación de protocolos de seguridad y encriptación.",
            "Gonzalo Fuentes",
            "gonzalo.fuentes@transbank.cl",
            "+56 2 2661 8000",
        ),
    ];

    (0..count)
        .map(|i| {
            let data = &practice_data[i % practice_data.len()];
            let start_days = (i as i64) * 30 + 30; // Prácticas escalonadas cada 30 días
            let duration_days = 180; // 6 meses de duración

            Practice {
                id: Uuid::new_v4(),
                enterprise_name: data.0.to_string(),
                location: data.1.to_string(),
                description: data.2.to_string(),
                supervisor_name: data.3.to_string(),
                supervisor_email: data.4.to_string(),
                supervisor_phone: data.5.to_string(),
                start_date: Some(Utc::now() + chrono::Duration::days(start_days)),
                end_date: Some(
                    Utc::now() + chrono::Duration::days(start_days + duration_days),
                ),
                practice_status: PracticeStatus::Pending,
            }
        })
        .collect()
}
