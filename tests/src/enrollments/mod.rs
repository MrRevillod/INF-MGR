use axum::http::StatusCode;
use serde_json::json;
use sword::web::ResponseBody;
use uuid::Uuid;

use crate::{
    courses::utils::{CourseBuilder, create_course, delete_course},
    enrollments::utils::{EnrollmentBuilder, create_enrollment, delete_enrollment},
    extract_resource_id, init_test_app,
    users::utils::{create_student, create_teacher, delete_user, generate_unique_email},
};

pub mod utils;

#[tokio::test]
pub async fn create_enrollment_and_practice() {
    let app = init_test_app().await;

    let student_id = create_student(&app, Some(generate_unique_email())).await; // alu
    let teacher_id = create_teacher(&app, Some(generate_unique_email())).await;

    let course_data = CourseBuilder::new(&teacher_id).build();

    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    assert_eq!(
        enrollment["courseId"].as_str().unwrap(),
        course_id.as_str(),
        "Course ID does not match"
    );

    let practice_data = json!({
        "enterpriseName": "Test Enterprise 6AM",
        "description": "Test Description 6AM",
        "location": "Test Location 6AM",
        "supervisorName": "Test Supervisor 6AM",
        "supervisorEmail": generate_unique_email(),
        "supervisorPhone": "+56912345678",
        "startDate": "2024-09-01T00:00:00Z",
        "endDate": "2024-12-15T00:00:00Z",
    });

    let create_practice_res = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    let json = create_practice_res.json::<ResponseBody>().data;

    let practice_id = extract_resource_id(&json);

    //approve practice
    let approve_practice_res = app
        .post(&format!("/enrollments/{enrollment_id}/practice/{practice_id}/approve",))
        .await;

    approve_practice_res.assert_status(StatusCode::OK);

    // Clean up
    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
pub async fn create_enrollment_and_decline_practice() {
    let app = init_test_app().await;

    let student_id = create_student(&app, Some(generate_unique_email())).await;
    let teacher_id = create_teacher(&app, Some(generate_unique_email())).await;

    let course_data = CourseBuilder::new(&teacher_id).build();

    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    assert_eq!(
        enrollment["courseId"].as_str().unwrap(),
        course_id.as_str(),
        "Course ID does not match"
    );

    let practice_data = json!({
        "enterpriseName": "Test Enterprise Decline",
        "description": "Test Description Decline",
        "location": "Test Location Decline",
        "supervisorName": "Test Supervisor Decline",
        "supervisorEmail": generate_unique_email(),
        "supervisorPhone": "+56912345678",
        "startDate": "2024-09-01T00:00:00Z",
        "endDate": "2024-12-15T00:00:00Z",
    });

    let create_practice_res = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    let json = create_practice_res.json::<ResponseBody>().data;

    let practice_id = extract_resource_id(&json);

    // Decline practice
    let decline_practice_res = app
        .post(&format!("/enrollments/{enrollment_id}/practice/{practice_id}/decline"))
        .await;

    decline_practice_res.assert_status(StatusCode::OK);

    // Clean up
    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
pub async fn decline_nonexistent_practice_should_fail() {
    let app = init_test_app().await;

    let student_id = create_student(&app, Some(generate_unique_email())).await;
    let teacher_id = create_teacher(&app, Some(generate_unique_email())).await;

    let course_data = CourseBuilder::new(&teacher_id).build();

    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    // Try to decline a non-existent practice
    let fake_practice_id = Uuid::new_v4();
    let decline_practice_res = app
        .post(&format!("/enrollments/{enrollment_id}/practice/{fake_practice_id}/decline"))
        .await;

    decline_practice_res.assert_status(StatusCode::NOT_FOUND);

    // Clean up
    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
pub async fn decline_already_approved_practice_should_fail() {
    let app = init_test_app().await;

    let student_id = create_student(&app, Some(generate_unique_email())).await;
    let teacher_id = create_teacher(&app, Some(generate_unique_email())).await;

    let course_data = CourseBuilder::new(&teacher_id).build();

    let course = create_course(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    let practice_data = json!({
        "enterpriseName": "Test Enterprise Already Approved",
        "description": "Test Description Already Approved",
        "location": "Test Location Already Approved",
        "supervisorName": "Test Supervisor Already Approved",
        "supervisorEmail": generate_unique_email(),
        "supervisorPhone": "+56912345678",
        "startDate": "2024-09-01T00:00:00Z",
        "endDate": "2024-12-15T00:00:00Z",
    });

    let create_practice_res = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    let json = create_practice_res.json::<ResponseBody>().data;

    let practice_id = extract_resource_id(&json);

    // First approve the practice
    let approve_practice_res = app
        .post(&format!("/enrollments/{enrollment_id}/practice/{practice_id}/approve"))
        .await;

    approve_practice_res.assert_status(StatusCode::OK);

    // Then try to decline the already approved practice (should fail)
    let decline_practice_res = app
        .post(&format!("/enrollments/{enrollment_id}/practice/{practice_id}/decline"))
        .await;

    decline_practice_res.assert_status(StatusCode::BAD_REQUEST);

    // Clean up
    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

// #[tokio::test]
// async fn create_and_delete_inscription_should_work() {
//     let app = init_test_app().await;

//     let student_id = create_student(&app).await;
//     let teacher_id = create_teacher(&app).await;
//     let coordinator_id = create_coordinator(&app).await;

//     let asignature_data =
//         CourseBuilder::new(&teacher_id, &coordinator_id).build();

//     dbg!("Creating asignature with data: {:?}", &asignature_data);

//     let asignature = create_course(&app, &asignature_data).await;
//     let course_id = extract_resource_id(&asignature);

//     let enrollment_data = EnrollmentBuilder::new()
//         .with_student_id(&student_id)
//         .with_course_id(&course_id)
//         .build();

//     let incription = create_enrollment(&app, &enrollment_data).await;
//     let enrollment_id = extract_resource_id(&incription);

//     assert_eq!(
//         incription["courseId"].as_str().unwrap(),
//         course_id.as_str(),
//         "Asignature ID does not match"
//     );

//     delete_enrollment(&app, &enrollment_id).await;
//     delete_course(&app, &course_id).await;

//     delete_user(&app, &student_id).await;
//     delete_user(&app, &teacher_id).await;
//     delete_user(&app, &coordinator_id).await;
// }

// #[tokio::test]
// async fn create_enrollment_without_asignature_should_fail() {
//     let app = init_test_app().await;

//     let student_id = create_student(&app).await;

//     let enrollment_data = EnrollmentBuilder::new()
//         .with_student_id(&student_id)
//         .build();

//     app.post("/courses")
//         .json(&enrollment_data)
//         .await
//         .assert_status(StatusCode::BAD_REQUEST);

//     delete_user(&app, &student_id).await;
// }

// #[tokio::test]
// async fn create_enrollment_without_student_should_fail() {
//     let app = init_test_app().await;

//     let teacher_id = create_teacher(&app).await;
//     let coordinator_id = create_coordinator(&app).await;

//     let asignature_data =
//         CourseBuilder::new(&teacher_id, &coordinator_id).build();

//     let asignature = create_course(&app, &asignature_data).await;
//     let course_id = extract_resource_id(&asignature);

//     let enrollment_data = EnrollmentBuilder::new()
//         .with_course_id(&course_id)
//         .build();

//     app.post("/courses")
//         .json(&enrollment_data)
//         .await
//         .assert_status(StatusCode::BAD_REQUEST);

//     delete_course(&app, &course_id).await;
//     delete_user(&app, &teacher_id).await;
//     delete_user(&app, &coordinator_id).await;
// }

// #[tokio::test]
// async fn create_enrollment_with_non_valid_uuids_should_fail() {
//     let app = init_test_app().await;

//     let enrollment_data = EnrollmentBuilder::new()
//         .with_student_id(&"invalid-uuid".to_string())
//         .with_course_id(&"invalid-uuid".to_string())
//         .build();

//     app.post("/inscriptions")
//         .json(&enrollment_data)
//         .await
//         .assert_status(StatusCode::BAD_REQUEST);
// }

// #[tokio::test]
// async fn create_enrollment_with_non_existent_student_should_fail() {
//     let app = init_test_app().await;

//     let teacher_id = create_teacher(&app).await;
//     let coordinator_id = create_coordinator(&app).await;

//     let asignature_data =
//         CourseBuilder::new(&teacher_id, &coordinator_id).build();

//     let asignature = create_course(&app, &asignature_data).await;
//     let course_id = extract_resource_id(&asignature);

//     let enrollment_data = EnrollmentBuilder::new()
//         .with_student_id(&Uuid::new_v4().to_string())
//         .with_course_id(&course_id)
//         .build();

//     app.post("/courses")
//         .json(&enrollment_data)
//         .await
//         .assert_status(StatusCode::BAD_REQUEST);

//     delete_course(&app, &course_id).await;

//     delete_user(&app, &teacher_id).await;
//     delete_user(&app, &coordinator_id).await;
// }

// #[tokio::test]
// async fn create_duplicate_inscription_should_fail() {
//     let app = init_test_app().await;

//     let student_id = create_student(&app).await;
//     let teacher_id = create_teacher(&app).await;
//     let coordinator_id = create_coordinator(&app).await;

//     let asignature_data =
//         CourseBuilder::new(&teacher_id, &coordinator_id).build();

//     let asignature = create_course(&app, &asignature_data).await;
//     let course_id = extract_resource_id(&asignature);

//     let enrollment_data = EnrollmentBuilder::new()
//         .with_student_id(&student_id)
//         .with_course_id(&course_id)
//         .build();

//     let inscription = create_enrollment(&app, &enrollment_data).await;
//     let enrollment_id = extract_resource_id(&inscription);

//     app.post("/inscriptions")
//         .json(&enrollment_data)
//         .await
//         .assert_status(StatusCode::CONFLICT);

//     delete_enrollment(&app, &enrollment_id).await;
//     delete_course(&app, &course_id).await;

//     delete_user(&app, &student_id).await;
//     delete_user(&app, &teacher_id).await;
//     delete_user(&app, &coordinator_id).await;
// }
