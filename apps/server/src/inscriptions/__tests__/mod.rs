use crate::tests::{generate_unique_code, generate_unique_name, init_test_app};
use serde_json::json;
use sword::web::ResponseBody;
use uuid::Uuid;

async fn create_test_teacher_and_student(
    app: &axum_test::TestServer,
) -> (String, String) {
    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test Teacher",
        "email": format!("teacher{}@example.com", Uuid::new_v4()),
        "role": "teacher",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let response = app.post("/users").json(&new_user).await;
    let body = response.json::<ResponseBody>();

    let teacher_id = body
        .data
        .get("id")
        .and_then(|id| id.as_str())
        .unwrap()
        .to_string();

    let new_student = json!({
        "rut": "21395308-7",
        "name": "Test Student",
        "email": format!("student{}@example.com", Uuid::new_v4()),
        "role": "student",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let response = app.post("/users").json(&new_student).await;
    let body = response.json::<ResponseBody>();
    let student_id = body
        .data
        .get("id")
        .and_then(|id| id.as_str())
        .unwrap()
        .to_string();

    (teacher_id, student_id)
}

async fn create_test_asignature_with_app(
    app: &axum_test::TestServer,
    teacher_id: &str,
) -> String {
    let new_asignature = json!({
        "year": 2024,
        "code": generate_unique_code(),
        "name": generate_unique_name(),
        "evaluations": [
            {
                "name": "Test Evaluation",
                "weight": 1.0
            }
        ],
        "teacherId": teacher_id
    });

    let response = app.post("/asignatures").json(&new_asignature).await;
    let body = response.json::<ResponseBody>();

    body.data
        .get("id")
        .and_then(|id| id.as_str())
        .unwrap()
        .to_string()
}

async fn cleanup_user_with_app(app: &axum_test::TestServer, user_id: &str) {
    app.delete(&format!("/users/{}", user_id)).await;
}

async fn cleanup_asignature_with_app(
    app: &axum_test::TestServer,
    asignature_id: &str,
) {
    app.delete(&format!("/asignatures/{}", asignature_id)).await;
}

// ==================== CRUD TESTS ====================

#[tokio::test]
async fn test_create_inscription_should_work() {
    let app = init_test_app().await;
    let (teacher_id, student_id) = create_test_teacher_and_student(&app).await;
    let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

    let new_inscription = json!({
        "userId": student_id,
        "asignatureId": asignature_id
    });

    let response = app.post("/inscriptions").json(&new_inscription).await;
    let body = response.json::<ResponseBody>();

    dbg!(&body);

    assert_eq!(response.status_code(), 201);
    assert!(body.success);
    assert!(body.data.get("id").is_some());
    assert_eq!(
        body.data.get("userId").and_then(|id| id.as_str()),
        Some(student_id.as_str())
    );
    assert_eq!(
        body.data.get("asignatureId").and_then(|id| id.as_str()),
        Some(asignature_id.as_str())
    );
    assert_eq!(
        body.data.get("status").and_then(|s| s.as_str()),
        Some("pending")
    );

    let inscription_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Cleanup
    app.delete(&format!("/inscriptions/{}", inscription_id))
        .await;
    cleanup_asignature_with_app(&app, &asignature_id).await;
    cleanup_user_with_app(&app, &teacher_id).await;
    cleanup_user_with_app(&app, &student_id).await;
}

#[tokio::test]
async fn test_get_inscriptions() {
    let app = init_test_app().await;

    let response = app.get("/inscriptions").await;
    let body = response.json::<ResponseBody>();

    assert_eq!(response.status_code(), 200);
    assert!(body.success);
    assert!(body.data.is_array());
}

// #[tokio::test]
// async fn test_get_inscriptions_with_filter() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Test filter by user_id
//     let response = app
//         .get(&format!("/inscriptions?userId={}", student_id))
//         .await;
//     assert_eq!(response.status_code(), 200);

//     let body = response.json::<ResponseBody>();
//     assert!(body.success);
//     assert!(body.data.is_array());

//     // Test filter by asignature_id
//     let response = app
//         .get(&format!("/inscriptions?asignatureId={}", asignature_id))
//         .await;
//     assert_eq!(response.status_code(), 200);

//     let body = response.json::<ResponseBody>();
//     assert!(body.success);
//     assert!(body.data.is_array());

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// #[tokio::test]
// async fn test_update_inscription() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Update inscription
//     let update_inscription = json!({
//         "status": "active",
//         "evaluationScores": [
//             {
//                 "id": Uuid::new_v4().to_string(),
//                 "score": 6.5
//             }
//         ]
//     });

//     let response = app
//         .patch(&format!("/inscriptions/{}", inscription_id))
//         .json(&update_inscription)
//         .await;

//     assert_eq!(response.status_code(), 200);

//     let body = response.json::<ResponseBody>();
//     assert!(body.success);
//     assert_eq!(
//         body.data.get("status").and_then(|s| s.as_str()),
//         Some("active")
//     );

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// #[tokio::test]
// async fn test_delete_inscription() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Delete inscription
//     let response = app
//         .delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     assert_eq!(response.status_code(), 200);

//     // Verify deletion - try to get the deleted inscription
//     let get_response = app.get(&format!("/inscriptions/{}", inscription_id)).await;
//     assert_eq!(get_response.status_code(), 404);

//     // Cleanup
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// ==================== CREATE INSCRIPTION VALIDATION TESTS ====================

// #[tokio::test]
// async fn test_create_inscription_invalid_user_id() {
//     let app = init_test_app().await;

//     let new_inscription = json!({
//         "userId": "invalid-uuid",
//         "asignatureId": Uuid::new_v4().to_string()
//     });

//     let response = app.post("/inscriptions").json(&new_inscription).await;
//     assert_eq!(response.status_code(), 400);

//     let body = response.json::<ResponseBody>();
//     assert!(!body.success);
// }

// #[tokio::test]
// async fn test_create_inscription_invalid_asignature_id() {
//     let app = init_test_app().await;

//     let new_inscription = json!({
//         "userId": Uuid::new_v4().to_string(),
//         "asignatureId": "invalid-uuid"
//     });

//     let response = app.post("/inscriptions").json(&new_inscription).await;
//     assert_eq!(response.status_code(), 400);

//     let body = response.json::<ResponseBody>();
//     assert!(!body.success);
// }

// #[tokio::test]
// async fn test_create_inscription_missing_user_id() {
//     let app = init_test_app().await;

//     let new_inscription = json!({
//         "asignatureId": Uuid::new_v4().to_string()
//     });

//     let response = app.post("/inscriptions").json(&new_inscription).await;
//     assert_eq!(response.status_code(), 400);

//     let body = response.json::<ResponseBody>();
//     assert!(!body.success);
// }

// #[tokio::test]
// async fn test_create_inscription_missing_asignature_id() {
//     let app = init_test_app().await;

//     let new_inscription = json!({
//         "userId": Uuid::new_v4().to_string()
//     });

//     let response = app.post("/inscriptions").json(&new_inscription).await;
//     assert_eq!(response.status_code(), 400);

//     let body = response.json::<ResponseBody>();
//     assert!(!body.success);
// }

// #[tokio::test]
// async fn test_create_inscription_nonexistent_user() {
//     let app = init_test_app().await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     let new_inscription = json!({
//         "userId": Uuid::new_v4().to_string(), // Non-existent user
//         "asignatureId": asignature_id
//     });

//     let response = app.post("/inscriptions").json(&new_inscription).await;
//     assert_eq!(response.status_code(), 404);

//     let body = response.json::<ResponseBody>();
//     assert!(!body.success);

//     // Cleanup
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
// }

// #[tokio::test]
// async fn test_create_inscription_nonexistent_asignature() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;

//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": Uuid::new_v4().to_string() // Non-existent asignature
//     });

//     let response = app.post("/inscriptions").json(&new_inscription).await;
//     assert_eq!(response.status_code(), 404);

//     let body = response.json::<ResponseBody>();
//     assert!(!body.success);

//     // Cleanup
//     cleanup_user_with_app(&app, &student_id).await;
// }

// #[tokio::test]
// async fn test_create_inscription_duplicate() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     // Create first inscription
//     let first_response = app.post("/inscriptions").json(&new_inscription).await;
//     assert_eq!(first_response.status_code(), 201);

//     let first_body = first_response.json::<ResponseBody>();
//     let inscription_id = first_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Try to create duplicate inscription
//     let second_response = app.post("/inscriptions").json(&new_inscription).await;
//     assert_eq!(second_response.status_code(), 409); // Conflict

//     let body = second_response.json::<ResponseBody>();
//     assert!(!body.success);

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// // ==================== UPDATE INSCRIPTION VALIDATION TESTS ====================

// #[tokio::test]
// async fn test_update_inscription_invalid_status() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Try to update with invalid status
//     let update_inscription = json!({
//         "status": "invalid_status"
//     });

//     let response = app
//         .patch(&format!("/inscriptions/{}", inscription_id))
//         .json(&update_inscription)
//         .await;

//     assert_eq!(response.status_code(), 400);

//     let body = response.json::<ResponseBody>();
//     assert!(!body.success);

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// #[tokio::test]
// async fn test_update_inscription_invalid_evaluation_score_too_low() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Try to update with invalid evaluation score
//     let update_inscription = json!({
//         "evaluationScores": [
//             {
//                 "id": Uuid::new_v4().to_string(),
//                 "score": 0.5 // Too low
//             }
//         ]
//     });

//     let response = app
//         .patch(&format!("/inscriptions/{}", inscription_id))
//         .json(&update_inscription)
//         .await;

//     assert_eq!(response.status_code(), 400);

//     let body = response.json::<ResponseBody>();
//     assert!(!body.success);

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// #[tokio::test]
// async fn test_update_inscription_invalid_evaluation_score_too_high() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Try to update with invalid evaluation score
//     let update_inscription = json!({
//         "evaluationScores": [
//             {
//                 "id": Uuid::new_v4().to_string(),
//                 "score": 7.5 // Too high
//             }
//         ]
//     });

//     let response = app
//         .patch(&format!("/inscriptions/{}", inscription_id))
//         .json(&update_inscription)
//         .await;

//     assert_eq!(response.status_code(), 400);

//     let body = response.json::<ResponseBody>();
//     assert!(!body.success);

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// #[tokio::test]
// async fn test_update_inscription_invalid_evaluation_id() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Try to update with invalid evaluation ID
//     let update_inscription = json!({
//         "evaluationScores": [
//             {
//                 "id": "invalid-uuid",
//                 "score": 6.0
//             }
//         ]
//     });

//     let response = app
//         .patch(&format!("/inscriptions/{}", inscription_id))
//         .json(&update_inscription)
//         .await;

//     assert_eq!(response.status_code(), 400);

//     let body = response.json::<ResponseBody>();
//     assert!(!body.success);

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// #[tokio::test]
// async fn test_update_inscription_empty_status() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Try to update with empty status
//     let update_inscription = json!({
//         "status": ""
//     });

//     let response = app
//         .patch(&format!("/inscriptions/{}", inscription_id))
//         .json(&update_inscription)
//         .await;

//     assert_eq!(response.status_code(), 400);

//     let body = response.json::<ResponseBody>();
//     assert!(!body.success);

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// // ==================== VALID STATUS TESTS ====================

// #[tokio::test]
// async fn test_update_inscription_valid_status_active() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Update with valid status
//     let update_inscription = json!({
//         "status": "active"
//     });

//     let response = app
//         .patch(&format!("/inscriptions/{}", inscription_id))
//         .json(&update_inscription)
//         .await;

//     assert_eq!(response.status_code(), 200);

//     let body = response.json::<ResponseBody>();
//     assert!(body.success);
//     assert_eq!(
//         body.data.get("status").and_then(|s| s.as_str()),
//         Some("active")
//     );

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// #[tokio::test]
// async fn test_update_inscription_valid_status_completed() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Update with valid status
//     let update_inscription = json!({
//         "status": "completed"
//     });

//     let response = app
//         .patch(&format!("/inscriptions/{}", inscription_id))
//         .json(&update_inscription)
//         .await;

//     assert_eq!(response.status_code(), 200);

//     let body = response.json::<ResponseBody>();
//     assert!(body.success);
//     assert_eq!(
//         body.data.get("status").and_then(|s| s.as_str()),
//         Some("completed")
//     );

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// #[tokio::test]
// async fn test_update_inscription_valid_status_evaluating() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Update with valid status
//     let update_inscription = json!({
//         "status": "evaluating"
//     });

//     let response = app
//         .patch(&format!("/inscriptions/{}", inscription_id))
//         .json(&update_inscription)
//         .await;

//     assert_eq!(response.status_code(), 200);

//     let body = response.json::<ResponseBody>();
//     assert!(body.success);
//     assert_eq!(
//         body.data.get("status").and_then(|s| s.as_str()),
//         Some("evaluating")
//     );

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }

// #[tokio::test]
// async fn test_update_inscription_valid_status_inactive() {
//     let app = init_test_app().await;
//     let student_id = create_test_user_with_app(&app, "student").await;
//     let teacher_id = create_test_user_with_app(&app, "teacher").await;
//     let asignature_id = create_test_asignature_with_app(&app, &teacher_id).await;

//     // Create inscription
//     let new_inscription = json!({
//         "userId": student_id,
//         "asignatureId": asignature_id
//     });

//     let create_response = app.post("/inscriptions").json(&new_inscription).await;
//     let create_body = create_response.json::<ResponseBody>();
//     let inscription_id = create_body
//         .data
//         .get("id")
//         .and_then(|id| id.as_str())
//         .unwrap();

//     // Update with valid status
//     let update_inscription = json!({
//         "status": "inactive"
//     });

//     let response = app
//         .patch(&format!("/inscriptions/{}", inscription_id))
//         .json(&update_inscription)
//         .await;

//     assert_eq!(response.status_code(), 200);

//     let body = response.json::<ResponseBody>();
//     assert!(body.success);
//     assert_eq!(
//         body.data.get("status").and_then(|s| s.as_str()),
//         Some("inactive")
//     );

//     // Cleanup
//     app.delete(&format!("/inscriptions/{}", inscription_id))
//         .await;
//     cleanup_asignature_with_app(&app, &asignature_id).await;
//     cleanup_user_with_app(&app, &teacher_id).await;
//     cleanup_user_with_app(&app, &student_id).await;
// }
