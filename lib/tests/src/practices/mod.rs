pub mod utils;

use crate::{
    courses::utils::{CourseBuilder, create_course, delete_course},
    enrollments::utils::{EnrollmentBuilder, create_enrollment, delete_enrollment},
    extract_resource_id, init_test_app,
    practices::utils::PracticeBuilder,
    users::utils::{create_student, create_teacher, delete_user},
};
use axum::http::StatusCode;
use chrono::{Duration, Utc};

#[tokio::test]
async fn create_practice_with_valid_phone_should_work() {
    let app = init_test_app().await;

    // Crear student y teacher
    let student_id =
        create_student(&app, Some("validphone_student@example.com".to_string()))
            .await;
    let teacher_id =
        create_teacher(&app, Some("validphone_teacher@example.com".to_string()))
            .await;

    // Crear course
    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    // Crear enrollment
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();
    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);
    let start_date = Utc::now();
    let end_date = start_date + Duration::days(90);

    // Crear practice con número válido
    let practice_data = PracticeBuilder::new()
        .with_enterprise_name("Empresa Test")
        .with_description("Descripción de prueba")
        .with_location("Ubicación de prueba")
        .with_supervisor_name("Supervisor Test")
        .with_supervisor_email("supervisor@example.com")
        .with_supervisor_phone("+56912345678") // válido
        .with_start_date(&start_date.to_rfc3339())
        .with_end_date(&end_date.to_rfc3339())
        .build();

    let response = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    response.assert_status(StatusCode::CREATED);

    // Limpieza
    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn create_practice_with_invalid_phone_should_fail() {
    let app = init_test_app().await;

    let student_id =
        create_student(&app, Some("invalidphone_student@example.com".to_string()))
            .await;
    let teacher_id =
        create_teacher(&app, Some("invalidphone_teacher@example.com".to_string()))
            .await;

    // Crear course
    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    // Crear enrollment
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();
    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);
    let start_date = Utc::now();
    let end_date = start_date + Duration::days(90);
    // Construimos la práctica con número inválido
    let practice_data = PracticeBuilder::new()
        .with_enterprise_name("Empresa Test")
        .with_description("Descripción de prueba")
        .with_location("Ubicación de prueba")
        .with_supervisor_name("Supervisor Test")
        .with_supervisor_email("supervisor@example.com")
        .with_supervisor_phone("12345") // inválido para tu regex
        .with_start_date(&start_date.to_rfc3339())
        .with_end_date(&end_date.to_rfc3339())
        .build();

    let response = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn create_practice_with_invalid_enterprise_name_should_fail() {
    let app = init_test_app().await;

    let student_id = create_student(
        &app,
        Some("invalid_enterprise_student@example.com".to_string()),
    )
    .await;
    let teacher_id = create_teacher(
        &app,
        Some("invalid_enterprise_teacher@example.com".to_string()),
    )
    .await;

    // Crear course
    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    // Crear enrollment
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();
    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);
    let start_date = Utc::now();
    let end_date = start_date + Duration::days(90);
    // Construimos la práctica con número inválido
    let practice_data = PracticeBuilder::new()
        .with_enterprise_name("En un pequeño pueblo llamado lolaso rodeado de montañas y ríos cristalinos, vivía una niña llamada Clara que soñaba con conocer el mundo más allá de los bosques y los campos que veía cada día. Cada mañana, despertaba con el canto de los pájaros y el aroma del pan recién horneado en la panadería del centro. Su curiosidad era insaciable y pasaba horas leyendo libros antiguos llenos de historias de reinos lejanos, mares infinitos y criaturas fantásticas. Clara imaginaba aventuras en las que cabalgaba dragones, navegaba océanos tempestuosos y resolvía misterios que nadie más podía comprender. Sus amigos del pueblo la miraban con asombro, preguntándose cómo alguien tan joven podía tener sueños tan grandes. Sin embargo, Clara no se dejaba intimidar; sabía que cada paso que daba, cada palabra que leía y cada historia que escuchaba la acercaban un poco más a su destino. Una tarde, mientras exploraba un sendero desconocido cerca del bosque, encontró un mapa antiguo escondido dentro de un tronco hueco. El mapa mostraba rutas secretas, cuevas misteriosas y tesoros olvidados. Con el corazón latiendo con fuerza, decidió emprender un viaje que cambiaría su vida para siempre. Reunió provisiones, despidió a sus padres y se adentró en la naturaleza con valentía y determinación. Cada día traía nuevos desafíos: ríos caudalosos, montañas escarpadas y animales salvajes que debía evitar o enfrentar. Pero Clara aprendía rápido, adaptándose a cada circunstancia y descubriendo habilidades que no sabía que tenía. Mientras avanzaba, también encontraba personas que necesitaban ayuda, y con su ingenio y bondad lograba resolver conflictos, ganar aliados y aprender lecciones valiosas sobre la vida y la amistad. Finalmente, tras semanas de aventura, Clara llegó a un valle escondido que parecía sacado de un cuento de hadas, donde comprendió que el verdadero tesoro no era el oro ni las gemas, sino la experiencia, los amigos y la valentía que había desarrollado durante su extraordinario viaje por el mundo desconocido.")
        .with_description("Descripción de prueba")
        .with_location("Ubicación de prueba")
        .with_supervisor_name("Supervisor Test")
        .with_supervisor_email("supervisor@example.com")
        .with_supervisor_phone("+56912345678") 
        .with_start_date(&start_date.to_rfc3339())
        .with_end_date(&end_date.to_rfc3339())
        .build();

    let response = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn create_practice_with_invalid_description_should_fail() {
    let app = init_test_app().await;

    let student_id =
        create_student(&app, Some("description_student@example.com".to_string()))
            .await;
    let teacher_id =
        create_teacher(&app, Some("description_teacher@example.com".to_string()))
            .await;

    // Crear course
    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    // Crear enrollment
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();
    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);
    let start_date = Utc::now();
    let end_date = start_date + Duration::days(90);
    // Construimos la práctica con número inválido
    let practice_data = PracticeBuilder::new()
        .with_enterprise_name("Empresa Test")
        .with_description("En un pequeño pueblo llamado lolaso rodeado de montañas y ríos cristalinos, vivía una niña llamada Clara que soñaba con conocer el mundo más allá de los bosques y los campos que veía cada día. Cada mañana, despertaba con el canto de los pájaros y el aroma del pan recién horneado en la panadería del centro. Su curiosidad era insaciable y pasaba horas leyendo libros antiguos llenos de historias de reinos lejanos, mares infinitos y criaturas fantásticas. Clara imaginaba aventuras en las que cabalgaba dragones, navegaba océanos tempestuosos y resolvía misterios que nadie más podía comprender. Sus amigos del pueblo la miraban con asombro, preguntándose cómo alguien tan joven podía tener sueños tan grandes. Sin embargo, Clara no se dejaba intimidar; sabía que cada paso que daba, cada palabra que leía y cada historia que escuchaba la acercaban un poco más a su destino. Una tarde, mientras exploraba un sendero desconocido cerca del bosque, encontró un mapa antiguo escondido dentro de un tronco hueco. El mapa mostraba rutas secretas, cuevas misteriosas y tesoros olvidados. Con el corazón latiendo con fuerza, decidió emprender un viaje que cambiaría su vida para siempre. Reunió provisiones, despidió a sus padres y se adentró en la naturaleza con valentía y determinación. Cada día traía nuevos desafíos: ríos caudalosos, montañas escarpadas y animales salvajes que debía evitar o enfrentar. Pero Clara aprendía rápido, adaptándose a cada circunstancia y descubriendo habilidades que no sabía que tenía. Mientras avanzaba, también encontraba personas que necesitaban ayuda, y con su ingenio y bondad lograba resolver conflictos, ganar aliados y aprender lecciones valiosas sobre la vida y la amistad. Finalmente, tras semanas de aventura, Clara llegó a un valle escondido que parecía sacado de un cuento de hadas, donde comprendió que el verdadero tesoro no era el oro ni las gemas, sino la experiencia, los amigos y la valentía que había desarrollado durante su extraordinario viaje por el mundo desconocido.")
        .with_location("Ubicación de prueba")
        .with_supervisor_name("Supervisor Test")
        .with_supervisor_email("supervisor@example.com")
        .with_supervisor_phone("+56912345678") 
        .with_start_date(&start_date.to_rfc3339())
        .with_end_date(&end_date.to_rfc3339())
        .build();

    let response = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn create_practice_with_invalid_location_should_fail() {
    let app = init_test_app().await;

    let student_id =
        create_student(&app, Some("location_student@example.com".to_string())).await;
    let teacher_id =
        create_teacher(&app, Some("location_teacher@example.com".to_string())).await;

    // Crear course
    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    // Crear enrollment
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();
    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);
    let start_date = Utc::now();
    let end_date = start_date + Duration::days(90);
    // Construimos la práctica con número inválido
    let practice_data = PracticeBuilder::new()
        .with_enterprise_name("Empresa Test")
        .with_description("Descripción de prueba")
        .with_location("En un pequeño pueblo llamado lolaso rodeado de montañas y ríos cristalinos, vivía una niña llamada Clara que soñaba con conocer el mundo más allá de los bosques y los campos que veía cada día. Cada mañana, despertaba con el canto de los pájaros y el aroma del pan recién horneado en la panadería del centro. Su curiosidad era insaciable y pasaba horas leyendo libros antiguos llenos de historias de reinos lejanos, mares infinitos y criaturas fantásticas. Clara imaginaba aventuras en las que cabalgaba dragones, navegaba océanos tempestuosos y resolvía misterios que nadie más podía comprender. Sus amigos del pueblo la miraban con asombro, preguntándose cómo alguien tan joven podía tener sueños tan grandes. Sin embargo, Clara no se dejaba intimidar; sabía que cada paso que daba, cada palabra que leía y cada historia que escuchaba la acercaban un poco más a su destino. Una tarde, mientras exploraba un sendero desconocido cerca del bosque, encontró un mapa antiguo escondido dentro de un tronco hueco. El mapa mostraba rutas secretas, cuevas misteriosas y tesoros olvidados. Con el corazón latiendo con fuerza, decidió emprender un viaje que cambiaría su vida para siempre. Reunió provisiones, despidió a sus padres y se adentró en la naturaleza con valentía y determinación. Cada día traía nuevos desafíos: ríos caudalosos, montañas escarpadas y animales salvajes que debía evitar o enfrentar. Pero Clara aprendía rápido, adaptándose a cada circunstancia y descubriendo habilidades que no sabía que tenía. Mientras avanzaba, también encontraba personas que necesitaban ayuda, y con su ingenio y bondad lograba resolver conflictos, ganar aliados y aprender lecciones valiosas sobre la vida y la amistad. Finalmente, tras semanas de aventura, Clara llegó a un valle escondido que parecía sacado de un cuento de hadas, donde comprendió que el verdadero tesoro no era el oro ni las gemas, sino la experiencia, los amigos y la valentía que había desarrollado durante su extraordinario viaje por el mundo desconocido.")
        .with_supervisor_name("Supervisor Test")
        .with_supervisor_email("supervisor@example.com")
        .with_supervisor_phone("+56912345678") 
        .with_start_date(&start_date.to_rfc3339())
        .with_end_date(&end_date.to_rfc3339())
        .build();

    let response = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn create_practice_with_invalid_supervisor_name_should_fail() {
    let app = init_test_app().await;

    let student_id = create_student(
        &app,
        Some("supervisor_name_student@example.com".to_string()),
    )
    .await;
    let teacher_id = create_teacher(
        &app,
        Some("supervisor_name_teacher@example.com".to_string()),
    )
    .await;

    // Crear course
    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    // Crear enrollment
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();
    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);
    let start_date = Utc::now();
    let end_date = start_date + Duration::days(90);
    // Construimos la práctica con número inválido
    let practice_data = PracticeBuilder::new()
        .with_enterprise_name("Empresa Test")
        .with_description("Descripción de prueba")
        .with_location("Ubicación de prueba")
        .with_supervisor_name("En un pequeño pueblo llamado lolaso rodeado de montañas y ríos cristalinos, vivía una niña llamada Clara que soñaba con conocer el mundo más allá de los bosques y los campos que veía cada día. Cada mañana, despertaba con el canto de los pájaros y el aroma del pan recién horneado en la panadería del centro. Su curiosidad era insaciable y pasaba horas leyendo libros antiguos llenos de historias de reinos lejanos, mares infinitos y criaturas fantásticas. Clara imaginaba aventuras en las que cabalgaba dragones, navegaba océanos tempestuosos y resolvía misterios que nadie más podía comprender. Sus amigos del pueblo la miraban con asombro, preguntándose cómo alguien tan joven podía tener sueños tan grandes. Sin embargo, Clara no se dejaba intimidar; sabía que cada paso que daba, cada palabra que leía y cada historia que escuchaba la acercaban un poco más a su destino. Una tarde, mientras exploraba un sendero desconocido cerca del bosque, encontró un mapa antiguo escondido dentro de un tronco hueco. El mapa mostraba rutas secretas, cuevas misteriosas y tesoros olvidados. Con el corazón latiendo con fuerza, decidió emprender un viaje que cambiaría su vida para siempre. Reunió provisiones, despidió a sus padres y se adentró en la naturaleza con valentía y determinación. Cada día traía nuevos desafíos: ríos caudalosos, montañas escarpadas y animales salvajes que debía evitar o enfrentar. Pero Clara aprendía rápido, adaptándose a cada circunstancia y descubriendo habilidades que no sabía que tenía. Mientras avanzaba, también encontraba personas que necesitaban ayuda, y con su ingenio y bondad lograba resolver conflictos, ganar aliados y aprender lecciones valiosas sobre la vida y la amistad. Finalmente, tras semanas de aventura, Clara llegó a un valle escondido que parecía sacado de un cuento de hadas, donde comprendió que el verdadero tesoro no era el oro ni las gemas, sino la experiencia, los amigos y la valentía que había desarrollado durante su extraordinario viaje por el mundo desconocido.")
        .with_supervisor_email("supervisor@example.com")
        .with_supervisor_phone("+56912345678") 
        .with_start_date(&start_date.to_rfc3339())
        .with_end_date(&end_date.to_rfc3339())
        .build();

    let response = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn create_practice_with_invalid_supervisor_email_should_fail() {
    let app = init_test_app().await;

    let student_id = create_student(
        &app,
        Some("supervisor_email_student@example.com".to_string()),
    )
    .await;
    let teacher_id = create_teacher(
        &app,
        Some("supervisor_email_teacher@example.com".to_string()),
    )
    .await;

    // Crear course
    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    // Crear enrollment
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();
    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);
    let start_date = Utc::now();
    let end_date = start_date + Duration::days(90);
    // Construimos la práctica con número inválido
    let practice_data = PracticeBuilder::new()
        .with_enterprise_name("Empresa Test")
        .with_description("Descripción de prueba")
        .with_location("Ubicación de prueba")
        .with_supervisor_name("Supervisor Test")
        .with_supervisor_email("supervisor.com")
        .with_supervisor_phone("+56912345678")
        .with_start_date(&start_date.to_rfc3339())
        .with_end_date(&end_date.to_rfc3339())
        .build();

    let response = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}
////////////////////////////////////////////////////////
#[tokio::test]
async fn create_practice_with_invalid_start_date_should_fail() {
    let app = init_test_app().await;

    let student_id =
        create_student(&app, Some("start_date_student@example.com".to_string()))
            .await;
    let teacher_id =
        create_teacher(&app, Some("start_date_teacher@example.com".to_string()))
            .await;

    // Crear course
    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    // Crear enrollment
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();
    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);
    let start_date = Utc::now();
    // Construimos la práctica con número inválido
    let practice_data = PracticeBuilder::new()
        .with_enterprise_name("Empresa Test")
        .with_description("Descripción de prueba")
        .with_location("Ubicación de prueba")
        .with_supervisor_name("Supervisor Test")
        .with_supervisor_email("supervisor@example.com")
        .with_supervisor_phone("+56912345678")
        .with_start_date(&start_date.to_rfc3339())
        .with_end_date(&start_date.to_rfc3339())
        .build();

    let response = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

///malo

#[tokio::test]
async fn create_practice_with_invalid_end_date_should_fail() {
    let app = init_test_app().await;

    let student_id =
        create_student(&app, Some("end_date_student@example.com".to_string())).await;
    let teacher_id =
        create_teacher(&app, Some("end_date_teacher@example.com".to_string())).await;

    // Crear course
    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    // Crear enrollment
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();
    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);
    let start_date = Utc::now();
    let end_date = start_date + Duration::days(90);
    // Construimos la práctica con número inválido
    let practice_data = PracticeBuilder::new()
        .with_enterprise_name("Empresa Test")
        .with_description("Descripción de prueba")
        .with_location("Ubicación de prueba")
        .with_supervisor_name("Supervisor Test")
        .with_supervisor_email("supervisor@example.com")
        .with_supervisor_phone("+56912345678")
        .with_start_date(&end_date.to_rfc3339())
        .with_end_date(&end_date.to_rfc3339())
        .build();

    let response = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

//malo

#[tokio::test]
async fn create_practice_with_invalid_start_date_delete_should_fail() {
    let app = init_test_app().await;

    let student_id = create_student(
        &app,
        Some("start_date_delete_student@example.com".to_string()),
    )
    .await;
    let teacher_id = create_teacher(
        &app,
        Some("start_date_delete_teacher@example.com".to_string()),
    )
    .await;

    // Crear course
    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    // Crear enrollment
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();
    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);
    let start_date = Utc::now();
    let end_date = start_date + Duration::days(90);
    // Construimos la práctica con número inválido
    let practice_data = PracticeBuilder::new()
        .with_enterprise_name("Empresa Test")
        .with_description("Descripción de prueba")
        .with_location("Ubicación de prueba")
        .with_supervisor_name("Supervisor Test")
        .with_supervisor_email("supervisor@example.com")
        .with_supervisor_phone("+56912345678")
        .with_start_date("2023-12")
        .with_end_date(&end_date.to_rfc3339())
        .build();

    let response = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn create_practice_with_invalid_end_date_delete_should_fail() {
    let app = init_test_app().await;

    let student_id = create_student(
        &app,
        Some("end_date_delete_student@example.com".to_string()),
    )
    .await;
    let teacher_id = create_teacher(
        &app,
        Some("end_date_delete_teacher@example.com".to_string()),
    )
    .await;

    // Crear course
    let course_data = CourseBuilder::new(&teacher_id).build();
    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    // Crear enrollment
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();
    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);
    let start_date = Utc::now();
    // Construimos la práctica con número inválido
    let practice_data = PracticeBuilder::new()
        .with_enterprise_name("Empresa Test")
        .with_description("Descripción de prueba")
        .with_location("Ubicación de prueba")
        .with_supervisor_name("Supervisor Test")
        .with_supervisor_email("supervisor@example.com")
        .with_supervisor_phone("+56912345678")
        .with_start_date(&start_date.to_rfc3339())
        .with_end_date("2023-12")
        .build();

    let response = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}
