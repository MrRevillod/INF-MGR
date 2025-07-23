use axum::http::StatusCode;
use uuid::Uuid;

use crate::tests::{
    app::{extract_resource_id, init_test_app},
    asignatures::utils::{create_asignature, delete_asignature, AsignatureBuilder},
    inscriptions::utils::{
        create_inscription, delete_incription, InscriptionBuilder,
    },
    users::utils::{create_student, create_teacher, delete_user},
};

pub mod utils;

#[tokio::test]
async fn create_and_delete_inscription_should_work() {
    let app = init_test_app().await;

    let student_id = create_student(&app).await;
    let teacher_id = create_teacher(&app).await;
    let asignature_data = AsignatureBuilder::new(&teacher_id).build();

    let asignature = create_asignature(&app, &asignature_data).await;
    let asignature_id = extract_resource_id(&asignature);

    let inscription_data = InscriptionBuilder::new()
        .with_student_id(&student_id)
        .with_asignature_id(&asignature_id)
        .build();

    let incription = create_inscription(&app, &inscription_data).await;
    let inscription_id = extract_resource_id(&incription);

    assert_eq!(
        incription["asignatureId"].as_str().unwrap(),
        asignature_id.as_str(),
        "Asignature ID does not match"
    );

    delete_incription(&app, &inscription_id).await;
    delete_asignature(&app, &asignature_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn create_inscription_without_asignature_should_fail() {
    let app = init_test_app().await;

    let student_id = create_student(&app).await;
    let inscription_data = InscriptionBuilder::new()
        .with_student_id(&student_id)
        .build();

    app.post("/inscriptions")
        .json(&inscription_data)
        .await
        .assert_status(StatusCode::BAD_REQUEST);

    delete_user(&app, &student_id).await;
}

#[tokio::test]
async fn create_inscription_without_student_should_fail() {
    let app = init_test_app().await;

    let teacher_id = create_teacher(&app).await;
    let asignature_data = AsignatureBuilder::new(&teacher_id).build();

    let asignature = create_asignature(&app, &asignature_data).await;
    let asignature_id = extract_resource_id(&asignature);

    let inscription_data = InscriptionBuilder::new()
        .with_asignature_id(&asignature_id)
        .build();

    app.post("/inscriptions")
        .json(&inscription_data)
        .await
        .assert_status(StatusCode::BAD_REQUEST);

    delete_asignature(&app, &asignature_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn create_inscription_with_non_valid_uuids_should_fail() {
    let app = init_test_app().await;

    let inscription_data = InscriptionBuilder::new()
        .with_student_id(&"invalid-uuid".to_string())
        .with_asignature_id(&"invalid-uuid".to_string())
        .build();

    app.post("/inscriptions")
        .json(&inscription_data)
        .await
        .assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_inscription_with_non_existent_student_should_fail() {
    let app = init_test_app().await;

    let teacher_id = create_teacher(&app).await;
    let asignature_data = AsignatureBuilder::new(&teacher_id).build();

    let asignature = create_asignature(&app, &asignature_data).await;
    let asignature_id = extract_resource_id(&asignature);

    let inscription_data = InscriptionBuilder::new()
        .with_student_id(&Uuid::new_v4().to_string())
        .with_asignature_id(&asignature_id)
        .build();

    app.post("/inscriptions")
        .json(&inscription_data)
        .await
        .assert_status(StatusCode::BAD_REQUEST);

    delete_asignature(&app, &asignature_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn create_duplicate_inscription_should_fail() {
    let app = init_test_app().await;

    let student_id = create_student(&app).await;
    let teacher_id = create_teacher(&app).await;
    let asignature_data = AsignatureBuilder::new(&teacher_id).build();

    let asignature = create_asignature(&app, &asignature_data).await;
    let asignature_id = extract_resource_id(&asignature);

    let inscription_data = InscriptionBuilder::new()
        .with_student_id(&student_id)
        .with_asignature_id(&asignature_id)
        .build();

    let inscription = create_inscription(&app, &inscription_data).await;
    let inscription_id = extract_resource_id(&inscription);

    app.post("/inscriptions")
        .json(&inscription_data)
        .await
        .assert_status(StatusCode::CONFLICT);

    delete_incription(&app, &inscription_id).await;
    delete_asignature(&app, &asignature_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}
