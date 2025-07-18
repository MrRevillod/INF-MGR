use crate::tests::init_test_app;
use serde_json::json;
use sword::web::ResponseBody;
use uuid::Uuid;

fn generate_unique_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

#[tokio::test]
async fn test_create_user_should_work() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let response = app.post("/users").json(&new_user).await;
    let body = response.json::<ResponseBody>();

    dbg!(&body);

    assert_eq!(response.status_code(), 201);

    let user_id = body
        .data
        .get("id")
        .and_then(|id| id.as_str())
        .expect("User ID should be present");

    let response = app.delete(&format!("/users/{}", user_id)).await;
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_get_users() {
    let app = init_test_app().await;
    let response = app.get("/users").await;

    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_update_user() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let create_response = app.post("/users").json(&new_user).await;
    let body = create_response.json::<ResponseBody>();

    assert_eq!(create_response.status_code(), 201);

    let user_id = body
        .data
        .get("id")
        .and_then(|id| id.as_str())
        .expect("User ID should be present");

    let new_email = generate_unique_email();

    let update_user = json!({
        "email": new_email
    });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 200);

    let updated_body = update_response.json::<ResponseBody>();

    let updated_user_email = updated_body
        .data
        .get("email")
        .and_then(|name| name.as_str())
        .expect("Updated user name should be present");

    assert_eq!(updated_user_email, new_email);

    let delete_response = app.delete(&format!("/users/{}", user_id)).await;
    assert_eq!(delete_response.status_code(), 200);
}

// ==================== CREATE USER VALIDATION TESTS ====================

#[tokio::test]
async fn test_create_user_invalid_rut_format() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499", // Missing dash and verification digit
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_invalid_rut_verification_digit() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-9", // Wrong verification digit (should be 7)
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_name_too_short() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test", // Only 4 characters, minimum is 5
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_name_too_long() {
    let app = init_test_app().await;

    let long_name = "a".repeat(101); // 101 characters, maximum is 100

    let new_user = json!({
        "rut": "34108499-7",
        "name": long_name,
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_invalid_email() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": "invalid-email", // Invalid email format
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_password_too_short() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "Test1!", // Only 6 characters, minimum is 8
        "confirmPassword": "Test1!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_password_missing_uppercase() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "testpassword123!", // Missing uppercase
        "confirmPassword": "testpassword123!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_password_missing_lowercase() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TESTPASSWORD123!", // Missing lowercase
        "confirmPassword": "TESTPASSWORD123!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_password_missing_digit() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword!", // Missing digit
        "confirmPassword": "TestPassword!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_password_missing_special_char() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123", // Missing special character
        "confirmPassword": "TestPassword123"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_passwords_dont_match() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "DifferentPassword123!" // Passwords don't match
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_invalid_role() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "invalid_role", // Invalid role
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_valid_roles() {
    let app = init_test_app().await;
    let valid_roles = vec![
        "administrator",
        "student",
        "teacher",
        "secretary",
        "coordinator",
    ];

    for role in valid_roles {
        let new_user = json!({
            "rut": "34108499-7",
            "name": "Test User",
            "email": generate_unique_email(),
            "role": role,
            "password": "TestPassword123!",
            "confirmPassword": "TestPassword123!"
        });

        let response = app.post("/users").json(&new_user).await;
        assert_eq!(response.status_code(), 201, "Role {} should be valid", role);

        // Clean up - delete the created user
        let body = response.json::<ResponseBody>();
        if let Some(user_id) = body.data.get("id").and_then(|id| id.as_str()) {
            app.delete(&format!("/users/{}", user_id)).await;
        }
    }
}

// ==================== UPDATE USER VALIDATION TESTS ====================

#[tokio::test]
async fn test_update_user_invalid_email() {
    let app = init_test_app().await;

    // First create a user
    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let create_response = app.post("/users").json(&new_user).await;
    let body = create_response.json::<ResponseBody>();
    let user_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Try to update with invalid email
    let update_user = json!({
        "email": "invalid-email-format"
    });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 400);

    // Clean up
    app.delete(&format!("/users/{}", user_id)).await;
}

#[tokio::test]
async fn test_update_user_password_only_one_field() {
    let app = init_test_app().await;

    // First create a user
    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let create_response = app.post("/users").json(&new_user).await;
    let body = create_response.json::<ResponseBody>();
    let user_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Try to update with only password (missing confirmPassword)
    let update_user = json!({
        "password": "NewPassword123!"
    });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 400);

    // Clean up
    app.delete(&format!("/users/{}", user_id)).await;
}

#[tokio::test]
async fn test_update_user_passwords_dont_match() {
    let app = init_test_app().await;

    // First create a user
    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let create_response = app.post("/users").json(&new_user).await;
    let body = create_response.json::<ResponseBody>();
    let user_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Try to update with mismatched passwords
    let update_user = json!({
        "password": "NewPassword123!",
        "confirmPassword": "DifferentPassword123!"
    });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 400);

    // Clean up
    app.delete(&format!("/users/{}", user_id)).await;
}

#[tokio::test]
async fn test_update_user_invalid_password_format() {
    let app = init_test_app().await;

    // First create a user
    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let create_response = app.post("/users").json(&new_user).await;
    let body = create_response.json::<ResponseBody>();
    let user_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Try to update with invalid password (too short)
    let update_user = json!({
        "password": "Test1!",
        "confirmPassword": "Test1!"
    });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 400);

    // Clean up
    app.delete(&format!("/users/{}", user_id)).await;
}

#[tokio::test]
async fn test_update_user_invalid_role() {
    let app = init_test_app().await;

    // First create a user
    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let create_response = app.post("/users").json(&new_user).await;
    let body = create_response.json::<ResponseBody>();
    let user_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Try to update with invalid role
    let update_user = json!({
        "role": "invalid_role"
    });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 400);

    // Clean up
    app.delete(&format!("/users/{}", user_id)).await;
}

#[tokio::test]
async fn test_update_user_valid_password_change() {
    let app = init_test_app().await;

    // First create a user
    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let create_response = app.post("/users").json(&new_user).await;
    let body = create_response.json::<ResponseBody>();
    let user_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Update with valid password
    let update_user = json!({
        "password": "NewValidPassword123!",
        "confirmPassword": "NewValidPassword123!"
    });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 200);

    // Clean up
    app.delete(&format!("/users/{}", user_id)).await;
}

#[tokio::test]
async fn test_update_user_valid_role_change() {
    let app = init_test_app().await;

    // First create a user
    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let create_response = app.post("/users").json(&new_user).await;
    let body = create_response.json::<ResponseBody>();
    let user_id = body.data.get("id").and_then(|id| id.as_str()).unwrap();

    // Update with valid role
    let update_user = json!({
        "role": "teacher"
    });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 200);

    let updated_body = update_response.json::<ResponseBody>();
    let updated_role = updated_body
        .data
        .get("role")
        .and_then(|role| role.as_str())
        .expect("Updated role should be present");

    assert_eq!(updated_role, "teacher");

    // Clean up
    app.delete(&format!("/users/{}", user_id)).await;
}

// ==================== EDGE CASES AND BOUNDARY TESTS ====================

#[tokio::test]
async fn test_create_user_name_boundary_valid() {
    let app = init_test_app().await;

    // Test minimum length (5 characters)
    let new_user = json!({
        "rut": "34108499-7",
        "name": "Tests", // Exactly 5 characters
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 201);

    // Clean up
    let body = response.json::<ResponseBody>();
    if let Some(user_id) = body.data.get("id").and_then(|id| id.as_str()) {
        app.delete(&format!("/users/{}", user_id)).await;
    }
}

#[tokio::test]
async fn test_create_user_name_boundary_maximum() {
    let app = init_test_app().await;

    let max_name = "a".repeat(100); // Exactly 100 characters

    let new_user = json!({
        "rut": "34108499-7",
        "name": max_name,
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPassword123!",
        "confirmPassword": "TestPassword123!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 201);

    // Clean up
    let body = response.json::<ResponseBody>();
    if let Some(user_id) = body.data.get("id").and_then(|id| id.as_str()) {
        app.delete(&format!("/users/{}", user_id)).await;
    }
}

#[tokio::test]
async fn test_create_user_password_boundary_minimum() {
    let app = init_test_app().await;

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": "TestPa1!", // Exactly 8 characters with all requirements
        "confirmPassword": "TestPa1!"
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 201);

    // Clean up
    let body = response.json::<ResponseBody>();
    if let Some(user_id) = body.data.get("id").and_then(|id| id.as_str()) {
        app.delete(&format!("/users/{}", user_id)).await;
    }
}

#[tokio::test]
async fn test_create_user_password_boundary_maximum() {
    let app = init_test_app().await;

    // Create a 100-character password with all requirements
    let base_password = "A".repeat(96); // 96 uppercase letters
    let password = format!("{}a1!", base_password); // Add lowercase, digit, and special char

    let new_user = json!({
        "rut": "34108499-7",
        "name": "Test User",
        "email": generate_unique_email(),
        "role": "administrator",
        "password": password,
        "confirmPassword": password
    });

    let response = app.post("/users").json(&new_user).await;
    assert_eq!(response.status_code(), 201);

    // Clean up
    let body = response.json::<ResponseBody>();
    if let Some(user_id) = body.data.get("id").and_then(|id| id.as_str()) {
        app.delete(&format!("/users/{}", user_id)).await;
    }
}
