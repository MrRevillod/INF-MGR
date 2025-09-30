use axum::http::StatusCode;
use sword::web::ResponseBody;
use uuid::Uuid;

use crate::{
    app::init_test_app,
    courses::utils::TestCourse,
    enrollments::utils::{EnrollmentBuilder, TestEnrollment},
    extract_resource_id,
    practices::utils::TestPractice,
    users::utils::TestUser,
};

pub mod utils;

#[tokio::test]
pub async fn create_enrollment_and_practice()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let student_id = TestUser::create_student(&app).await;
    let teacher_id = TestUser::create_teacher(&app).await;

    let course_data = TestCourse::builder(&teacher_id).build();

    let course = TestCourse::create(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = TestEnrollment::create(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    assert_eq!(
        enrollment["courseId"].as_str().unwrap(),
        course_id.as_str(),
        "Course ID does not match"
    );

    let practice_data = TestPractice::builder()
        .with_enterprise_name("Test Enterprise 6AM")
        .with_description("Test Description 6AM")
        .with_location("Test Location 6AM")
        .with_supervisor_name("Test Supervisor 6AM")
        .with_supervisor_phone("+56912345678")
        .with_start_date("2024-09-01T00:00:00Z")
        .with_end_date("2024-12-15T00:00:00Z")
        .build();

    let practice_id =
        TestPractice::create(&app, &enrollment_id, practice_data).await;

    TestPractice::approve(&app, &enrollment_id, &practice_id).await;

    TestPractice::delete(&app, &practice_id).await;
    TestEnrollment::delete(&app, &enrollment_id).await;
    TestCourse::delete(&app, &course_id).await;
    TestUser::delete(&app, &student_id).await;
    TestUser::delete(&app, &teacher_id).await;

    Ok(())
}

#[tokio::test]
pub async fn practice_approve_and_update_auth_doc()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let student_id = TestUser::create_student(&app).await;
    let teacher_id = TestUser::create_teacher(&app).await;

    let course_data = TestCourse::builder(&teacher_id).build();

    let course = TestCourse::create(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = TestEnrollment::create(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    assert_eq!(
        enrollment["courseId"].as_str().unwrap(),
        course_id.as_str(),
        "Course ID does not match"
    );

    let practice = TestPractice::builder()
        .with_enterprise_name("Test Enterprise 6AM")
        .with_description("Test Description 6AM")
        .with_location("Test Location 6AM")
        .with_supervisor_name("Test Supervisor 6AM")
        .with_supervisor_phone("+56912345678")
        .with_start_date("2024-09-01T00:00:00Z")
        .with_end_date("2024-12-15T00:00:00Z")
        .build();

    let practice_id = TestPractice::create(&app, &enrollment_id, practice).await;

    TestPractice::approve(&app, &enrollment_id, &practice_id).await;
    TestPractice::authorize(&app, &enrollment_id, &practice_id).await;

    TestPractice::delete(&app, &practice_id).await;
    TestEnrollment::delete(&app, &enrollment_id).await;

    TestCourse::delete(&app, &course_id).await;

    TestUser::delete(&app, &student_id).await;
    TestUser::delete(&app, &teacher_id).await;

    Ok(())
}

#[tokio::test]
pub async fn create_enrollment_and_decline_practice()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let student_id = TestUser::create_student(&app).await;
    let teacher_id = TestUser::create_teacher(&app).await;

    let course_data = TestCourse::builder(&teacher_id).build();

    let course = TestCourse::create(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = TestEnrollment::create(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    assert_eq!(
        enrollment["courseId"].as_str().unwrap(),
        course_id.as_str(),
        "Course ID does not match"
    );

    let practice_data = TestPractice::builder()
        .with_enterprise_name("Test Enterprise Decline")
        .with_description("Test Description Decline")
        .with_location("Test Location Decline")
        .with_supervisor_name("Test Supervisor Decline")
        .with_supervisor_phone("+56912345678")
        .with_start_date("2024-09-01T00:00:00Z")
        .with_end_date("2024-12-15T00:00:00Z")
        .build();

    let create_practice_res = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    let json = create_practice_res.json::<ResponseBody>().data;

    assert!(json.is_some(), "Response data should not be None");

    let practice_id = extract_resource_id(&json.unwrap());

    let decline_practice_res = app
        .post(&format!(
            "/enrollments/{enrollment_id}/practice/{practice_id}/decline"
        ))
        .await;

    decline_practice_res.assert_status(StatusCode::OK);

    TestEnrollment::delete(&app, &enrollment_id).await;
    TestCourse::delete(&app, &course_id).await;
    TestUser::delete(&app, &student_id).await;
    TestUser::delete(&app, &teacher_id).await;

    Ok(())
}

#[tokio::test]
pub async fn decline_nonexistent_practice_should_fail()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let student_id = TestUser::create_student(&app).await;
    let teacher_id = TestUser::create_teacher(&app).await;

    let course_data = TestCourse::builder(&teacher_id).build();

    let course = TestCourse::create(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = TestEnrollment::create(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    let fake_practice_id = Uuid::new_v4();
    let decline_practice_res = app
        .post(&format!(
            "/enrollments/{enrollment_id}/practice/{fake_practice_id}/decline"
        ))
        .await;

    decline_practice_res.assert_status(StatusCode::NOT_FOUND);

    TestEnrollment::delete(&app, &enrollment_id).await;
    TestCourse::delete(&app, &course_id).await;
    TestUser::delete(&app, &student_id).await;
    TestUser::delete(&app, &teacher_id).await;

    Ok(())
}

#[tokio::test]
pub async fn decline_already_approved_practice_should_fail()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let student_id = TestUser::create_student(&app).await;
    let teacher_id = TestUser::create_teacher(&app).await;

    let course_data = TestCourse::builder(&teacher_id).build();

    let course = TestCourse::create(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = TestEnrollment::create(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    let practice_data = TestPractice::builder()
        .with_enterprise_name("Test Enterprise Already Approved")
        .with_description("Test Description Already Approved")
        .with_location("Test Location Already Approved")
        .with_supervisor_name("Test Supervisor Already Approved")
        .with_supervisor_phone("+56912345678")
        .with_start_date("2024-09-01T00:00:00Z")
        .with_end_date("2024-12-15T00:00:00Z")
        .build();

    let create_practice_res = app
        .post(&format!("/enrollments/{enrollment_id}/practice"))
        .json(&practice_data)
        .await;

    let json = create_practice_res.json::<ResponseBody>().data;

    assert!(json.is_some(), "Response data should not be None");

    let practice_id = extract_resource_id(&json.unwrap());

    let approve_practice_res = app
        .post(&format!(
            "/enrollments/{enrollment_id}/practice/{practice_id}/approve"
        ))
        .await;

    approve_practice_res.assert_status(StatusCode::OK);

    let decline_practice_res = app
        .post(&format!(
            "/enrollments/{enrollment_id}/practice/{practice_id}/decline"
        ))
        .await;

    decline_practice_res.assert_status(StatusCode::BAD_REQUEST);

    TestEnrollment::delete(&app, &enrollment_id).await;
    TestCourse::delete(&app, &course_id).await;
    TestUser::delete(&app, &student_id).await;
    TestUser::delete(&app, &teacher_id).await;

    Ok(())
}

#[tokio::test]
pub async fn evaluate_practice_with_valid_grade_should_succeed()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let student_id = TestUser::create_student(&app).await;
    let teacher_id = TestUser::create_teacher(&app).await;

    let course_data = TestCourse::builder(&teacher_id)
        .with_evaluations(vec![("Evaluación Empresa", 40), ("Informe Final", 60)])
        .build();

    let course = TestCourse::create(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = TestEnrollment::create(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    let practice_data = TestPractice::builder()
        .with_enterprise_name("Test Enterprise Evaluation")
        .with_description("Test Description Evaluation")
        .with_location("Test Location Evaluation")
        .with_supervisor_name("Test Supervisor Evaluation")
        .with_supervisor_phone("+56912345678")
        .with_start_date("2024-09-01T00:00:00Z")
        .with_end_date("2024-12-15T00:00:00Z")
        .build();

    let practice_id =
        TestPractice::create(&app, &enrollment_id, practice_data).await;

    TestPractice::approve(&app, &enrollment_id, &practice_id).await;

    let evaluation_id = course["evaluations"][0]["id"].as_str().unwrap();

    TestPractice::evaluate(
        &app,
        &enrollment_id,
        &practice_id,
        &evaluation_id.to_string(),
        6.5,
    )
    .await;

    TestPractice::delete(&app, &practice_id).await;
    TestEnrollment::delete(&app, &enrollment_id).await;
    TestCourse::delete(&app, &course_id).await;
    TestUser::delete(&app, &student_id).await;
    TestUser::delete(&app, &teacher_id).await;

    Ok(())
}

#[tokio::test]
pub async fn evaluate_practice_with_invalid_grade_should_fail()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let student_id = TestUser::create_student(&app).await;
    let teacher_id = TestUser::create_teacher(&app).await;

    let course_data = TestCourse::builder(&teacher_id)
        .with_evaluations(vec![("Evaluación Empresa", 40), ("Informe Final", 60)])
        .build();

    let course = TestCourse::create(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = TestEnrollment::create(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    let practice_data = TestPractice::builder()
        .with_enterprise_name("Test Enterprise Invalid Grade")
        .with_description("Test Description Invalid Grade")
        .with_location("Test Location Invalid Grade")
        .with_supervisor_name("Test Supervisor Invalid Grade")
        .with_supervisor_phone("+56912345678")
        .with_start_date("2024-09-01T00:00:00Z")
        .with_end_date("2024-12-15T00:00:00Z")
        .build();

    let practice_id =
        TestPractice::create(&app, &enrollment_id, practice_data).await;

    // Approve practice first
    TestPractice::approve(&app, &enrollment_id, &practice_id).await;

    // Get the evaluation ID from course evaluations
    let evaluation_id = course["evaluations"][0]["id"].as_str().unwrap();

    // Test with grade below minimum (less than 1.0)
    let response_low = TestPractice::evaluate_without_assert(
        &app,
        &enrollment_id,
        &practice_id,
        &evaluation_id.to_string(),
        0.5,
    )
    .await;
    response_low.assert_status(StatusCode::BAD_REQUEST);

    // Test with grade above maximum (greater than 7.0)
    let response_high = TestPractice::evaluate_without_assert(
        &app,
        &enrollment_id,
        &practice_id,
        &evaluation_id.to_string(),
        8.0,
    )
    .await;
    response_high.assert_status(StatusCode::BAD_REQUEST);

    TestPractice::delete(&app, &practice_id).await;
    TestEnrollment::delete(&app, &enrollment_id).await;
    TestCourse::delete(&app, &course_id).await;
    TestUser::delete(&app, &student_id).await;
    TestUser::delete(&app, &teacher_id).await;

    Ok(())
}

#[tokio::test]
pub async fn evaluate_practice_with_custom_validation_should_fail()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let student_id = TestUser::create_student(&app).await;
    let teacher_id = TestUser::create_teacher(&app).await;

    let course_data = TestCourse::builder(&teacher_id)
        .with_evaluations(vec![("Evaluación Empresa", 40), ("Informe Final", 60)])
        .build();

    let course = TestCourse::create(&app, &course_data).await;
    let course_id = extract_resource_id(&course);

    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&course_id)
        .build();

    let enrollment = TestEnrollment::create(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&enrollment);

    let practice_data = TestPractice::builder()
        .with_enterprise_name("Test Enterprise Custom Validation")
        .with_description("Test Description Custom Validation")
        .with_location("Test Location Custom Validation")
        .with_supervisor_name("Test Supervisor Custom Validation")
        .with_supervisor_phone("+56912345678")
        .with_start_date("2024-09-01T00:00:00Z")
        .with_end_date("2024-12-15T00:00:00Z")
        .build();

    let practice_id =
        TestPractice::create(&app, &enrollment_id, practice_data).await;

    // Approve practice first
    TestPractice::approve(&app, &enrollment_id, &practice_id).await;

    // Get the evaluation ID from course evaluations
    let evaluation_id = course["evaluations"][0]["id"].as_str().unwrap();

    // Test with negative score
    let response_negative = TestPractice::evaluate_without_assert(
        &app,
        &enrollment_id,
        &practice_id,
        &evaluation_id.to_string(),
        -1.5,
    )
    .await;
    response_negative.assert_status(StatusCode::BAD_REQUEST);

    // Test with score having more than 1 decimal place
    let response_decimals = TestPractice::evaluate_without_assert(
        &app,
        &enrollment_id,
        &practice_id,
        &evaluation_id.to_string(),
        6.25, // 2 decimals should fail
    )
    .await;
    response_decimals.assert_status(StatusCode::BAD_REQUEST);

    // Test with valid score having exactly 1 decimal
    let response_valid_decimal = TestPractice::evaluate_without_assert(
        &app,
        &enrollment_id,
        &practice_id,
        &evaluation_id.to_string(),
        6.5,
    )
    .await;
    response_valid_decimal.assert_status(StatusCode::OK);

    // Test with valid integer score
    let response_valid_integer = TestPractice::evaluate_without_assert(
        &app,
        &enrollment_id,
        &practice_id,
        &evaluation_id.to_string(),
        5.0,
    )
    .await;
    response_valid_integer.assert_status(StatusCode::OK);

    TestPractice::delete(&app, &practice_id).await;
    TestEnrollment::delete(&app, &enrollment_id).await;
    TestCourse::delete(&app, &course_id).await;
    TestUser::delete(&app, &student_id).await;
    TestUser::delete(&app, &teacher_id).await;

    Ok(())
}
