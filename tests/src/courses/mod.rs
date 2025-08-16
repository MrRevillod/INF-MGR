pub mod utils;

use crate::{
    enrollments::utils::{EnrollmentBuilder, create_enrollment, delete_enrollment},
    extract_resource_id, init_test_app,
    users::utils::{create_student, create_teacher, delete_user},
};

use serde_json::json;
use sword::web::ResponseBody;
use utils::*;
use uuid::Uuid;

// ==================== CRUD TESTS ====================

#[tokio::test]
async fn test_create_course_should_work() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = CourseBuilder::new(&teacher_id).build();

    let created_course = create_course(&app, &new_course).await;
    let created_course_id = extract_resource_id(&created_course);

    assert_eq!(created_course.get("year").and_then(|y| y.as_i64()), Some(2024));
    assert_eq!(
        created_course.get("code").and_then(|c| c.as_str()),
        new_course.get("code").and_then(|c| c.as_str())
    );

    delete_course(&app, &created_course_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_get_courses() {
    let app = init_test_app().await;

    let response = app.get("/courses").await;
    assert_eq!(response.status_code(), 200);

    let body = response.json::<ResponseBody>();
    assert!(body.data.is_array());
}

#[tokio::test]
async fn test_update_course() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = CourseBuilder::new(&teacher_id)
        .with_evaluations(vec![("Bitácoras Semanales", 50), ("Informe Final", 50)])
        .build();

    let created_course = create_course(&app, &new_course).await;
    let created_course_id = extract_resource_id(&created_course);

    let new_teacher_id = create_teacher(&app, None).await;

    let update_course_data = json!({
        "teacherId": new_teacher_id.to_string(),
    });

    let updated_asignature = update_course(&app, &created_course_id, &update_course_data).await;

    assert_eq!(
        updated_asignature.get("teacherId").and_then(|t| t.as_str()),
        Some(new_teacher_id.to_string().as_str())
    );

    delete_course(&app, &created_course_id).await;

    delete_user(&app, &teacher_id).await;

    delete_user(&app, &new_teacher_id).await;
}

#[tokio::test]
async fn test_delete_course() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = CourseBuilder::new(&teacher_id)
        .with_single_evaluation("Test Evaluation", 100)
        .build();

    let created_course = create_course(&app, &new_course).await;
    let created_course_id = extract_resource_id(&created_course);

    delete_course(&app, &created_course_id).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_delete_course_with_active_inscriptions_should_fail() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = CourseBuilder::new(&teacher_id)
        .with_single_evaluation("Test Evaluation", 100)
        .build();

    let created_course = create_course(&app, &new_course).await;
    let created_course_id = extract_resource_id(&created_course);

    let student_id = create_student(&app, None).await;
    let enrollment_data = EnrollmentBuilder::new()
        .with_student_id(&student_id)
        .with_course_id(&created_course_id)
        .build();

    let inscription = create_enrollment(&app, &enrollment_data).await;
    let enrollment_id = extract_resource_id(&inscription);

    let response = app.delete(&format!("/courses/{created_course_id}")).await;

    assert_eq!(response.status_code(), 400);

    delete_enrollment(&app, &enrollment_id).await;
    delete_course(&app, &created_course_id).await;
    delete_user(&app, &student_id).await;
    delete_user(&app, &teacher_id).await;
}

// // ==================== CREATE ASIGNATURE VALIDATION TESTS ====================

#[tokio::test]
async fn test_create_course_invalid_year_too_low() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = CourseBuilder::new(&teacher_id).with_year(1999).build();

    let response = app.post("/courses").json(&new_course).await;
    let body = response.json::<ResponseBody>();

    assert_eq!(response.status_code(), 400);

    let error_arr = body
        .data
        .get("errors")
        .and_then(|e| e.as_array())
        .expect("Response data should be an array");

    assert!(!error_arr.is_empty(), "Expected validation errors");

    let year_error = error_arr
        .iter()
        .find(|err| err.get("field").and_then(|m| m.as_str()) == Some("year"));

    assert!(year_error.is_some(), "Expected a validation error for 'year'");

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_invalid_year_too_high() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = json!({
        "year": 2101,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_invalid_code_format() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = json!({
        "year": 2024,
        "code": "INVALID",
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);

    let body = response.json::<ResponseBody>();

    let error_arr = body
        .data
        .get("errors")
        .and_then(|e| e.as_array())
        .expect("Response data should be an array");

    assert!(!error_arr.is_empty(), "Expected validation errors");
    assert_eq!(error_arr[0].get("field").and_then(|m| m.as_str()), Some("code"),);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_invalid_code_length() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = json!({
        "year": 2024,
        "code": "INFO12345",
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_name_too_short() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": "",
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_name_too_long() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let long_name = "a".repeat(101);

    let new_course = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": long_name,
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_no_evaluations() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_evaluation_weights_not_sum_to_one() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation 1",
                "weight": 30
            },
            {
                "name": "Test Evaluation 2",
                "weight": 40 // Total: 70 (should be 100)
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_evaluation_name_too_short() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "", // Empty evaluation name
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_evaluation_name_too_long() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let long_evaluation_name = "a".repeat(101); // 101 characters, maximum is 100

    let new_course = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": long_evaluation_name,
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_evaluation_weight_too_low() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 0 // Below minimum (1)
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_evaluation_weight_too_high() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let new_course = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 101 // Above maximum (100)
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_invalid_teacher_id() {
    let app = init_test_app().await;

    let new_course = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": "invalid-uuid",
    });

    let response = app.post("/courses").json(&new_course).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_course_duplicate() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    let code = generate_unique_code();
    let name = generate_unique_course_name();

    let asignature_data = json!({
        "year": 2024,
        "code": code,
        "name": name,
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    // Create first asignature
    let response1 = app.post("/courses").json(&asignature_data).await;
    assert_eq!(response1.status_code(), 201);

    let body1 = response1.json::<ResponseBody>();
    let course_id = body1.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Try to create duplicate
    let response2 = app.post("/courses").json(&asignature_data).await;
    assert_eq!(response2.status_code(), 409); // Conflict

    // Cleanup
    app.delete(&format!("/courses/{}", course_id)).await;

    delete_user(&app, &teacher_id).await;
}

// // ==================== UPDATE ASIGNATURE VALIDATION TESTS ====================

#[tokio::test]
async fn test_update_course_invalid_teacher_id() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    // Create asignature
    let new_course = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let create_response = app.post("/courses").json(&new_course).await;
    let body = create_response.json::<ResponseBody>();
    let course_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Try to update with invalid teacher_id
    let update_course = json!({
        "teacherId": "invalid-uuid"
    });

    let update_response = app.patch(&format!("/courses/{}", course_id)).json(&update_course).await;

    assert_eq!(update_response.status_code(), 400);

    // Cleanup
    app.delete(&format!("/courses/{}", course_id)).await;
    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_update_nonexistent_asignature() {
    let app = init_test_app().await;

    let update_course = json!({
        "year": 2025
    });

    let fake_id = Uuid::new_v4();
    let update_response = app.patch(&format!("/courses/{}", fake_id)).json(&update_course).await;

    assert_eq!(update_response.status_code(), 404);
}

// // ==================== BOUNDARY TESTS ====================

#[tokio::test]
async fn test_create_course_valid_year_boundaries() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    // Test minimum year (2000)
    let min_year_asignature = json!({
        "year": 2000,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&min_year_asignature).await;
    assert_eq!(response.status_code(), 201);

    let body = response.json::<ResponseBody>();
    let min_course_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    app.delete(&format!("/courses/{}", min_course_id)).await;

    // other limit

    // Test minimum year (2000)
    let min_year_asignature = json!({
        "year": 2100,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&min_year_asignature).await;
    assert_eq!(response.status_code(), 201);

    let body = response.json::<ResponseBody>();
    let min_course_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    app.delete(&format!("/courses/{}", min_course_id)).await;

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_valid_name_boundaries() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    // Test minimum length (1 character)
    let min_name_asignature = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": "A", // Exactly 1 character
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&min_name_asignature).await;
    assert_eq!(response.status_code(), 201);

    let body = response.json::<ResponseBody>();
    let min_course_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Test maximum length (100 characters)
    let max_name = "A".repeat(100);
    let max_name_asignature = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": max_name,
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 100
            }
        ],
        "teacherId": teacher_id,
    });

    let response2 = app.post("/courses").json(&max_name_asignature).await;
    assert_eq!(response2.status_code(), 201);

    let body2 = response2.json::<ResponseBody>();
    let max_course_id = body2.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Cleanup
    app.delete(&format!("/courses/{}", min_course_id)).await;
    app.delete(&format!("/courses/{}", max_course_id)).await;

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_valid_evaluation_weight_boundaries() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    // Test minimum weight (1)
    let min_weight_asignature = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation 1",
                "weight": 1
            },
            {
                "name": "Test Evaluation 2",
                "weight": 99
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&min_weight_asignature).await;
    assert_eq!(response.status_code(), 201);

    let body = response.json::<ResponseBody>();
    let course_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Cleanup
    app.delete(&format!("/courses/{}", course_id)).await;

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_valid_evaluation_weight_boundaries_3_33() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    // Test weights distribution (33-33-34)
    let min_weight_asignature = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation 1",
                "weight": 33
            },
            {
                "name": "Test Evaluation 2",
                "weight": 33
            },
            {
                "name": "Test Evaluation 3",
                "weight": 33
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&min_weight_asignature).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}

#[tokio::test]
async fn test_create_course_valid_evaluation_but_repeated_names() {
    let app = init_test_app().await;
    let teacher_id = create_teacher(&app, None).await;

    // Test repeated evaluation names
    let min_weight_asignature = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_course_name(),
        "evaluations": [
            {
                "name": "Test Evaluation 1",
                "weight": 33
            },
            {
                "name": "Test Evaluation 2",
                "weight": 33
            },
            {
                "name": "Test Evaluation 1",
                "weight": 33
            }
        ],
        "teacherId": teacher_id,
    });

    let response = app.post("/courses").json(&min_weight_asignature).await;
    assert_eq!(response.status_code(), 400);

    delete_user(&app, &teacher_id).await;
}
