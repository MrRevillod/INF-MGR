pub mod utils;

use serde_json::json;
use sword::web::ResponseBody;

use crate::tests::init_test_app;

use utils::*;

#[tokio::test]
async fn test_create_user_should_work() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().build();

    let body = create_user(&app, new_user).await;

    let user_id = body
        .get("id")
        .and_then(|id| id.as_str())
        .expect("User ID should be present");

    delete_user(&app, user_id).await;
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
    let new_user = UserBuilder::new().build();
    let body = create_user(&app, new_user).await;

    let user_id = body
        .get("id")
        .and_then(|id| id.as_str())
        .expect("User ID should be present");

    let new_email = generate_unique_email();
    let update_user = json!({ "email": new_email });

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
    delete_user(&app, user_id).await;
}

// ==================== CREATE USER VALIDATION TESTS ====================

#[tokio::test]
async fn test_create_user_invalid_rut_format() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().with_rut("34108499").build();

    let response = app.post("/users").json(&new_user).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_invalid_rut_verification_digit() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().with_rut("34108499-9").build();
    let response = app.post("/users").json(&new_user).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_name_too_short() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().with_name("Test").build();
    let response = app.post("/users").json(&new_user).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_name_too_long() {
    let app = init_test_app().await;
    let long_name = "a".repeat(101);
    let new_user = UserBuilder::new().with_name(&long_name).build();
    let response = app.post("/users").json(&new_user).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_invalid_email() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().with_email("invalid-email").build();
    let response = app.post("/users").json(&new_user).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_password_too_short() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().with_password("Test1!", "Test1!").build();
    let response = app.post("/users").json(&new_user).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_password_missing_uppercase() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new()
        .with_password("testpassword123!", "testpassword123!")
        .build();

    let response = app.post("/users").json(&new_user).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_password_missing_lowercase() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new()
        .with_password("TESTPASSWORD123!", "TESTPASSWORD123!")
        .build();

    let response = app.post("/users").json(&new_user).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_password_missing_digit() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new()
        .with_password("TestPassword!", "TestPassword!")
        .build();

    let response = app.post("/users").json(&new_user).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_password_missing_special_char() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new()
        .with_password("TestPassword123", "TestPassword123")
        .build();

    let response = app.post("/users").json(&new_user).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_passwords_dont_match() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new()
        .with_password("TestPassword123!", "DifferentPassword123!")
        .build();

    let response = app.post("/users").json(&new_user).await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_create_user_invalid_role() {
    let app = init_test_app().await;

    let new_user = UserBuilder::new().with_roles(vec!["invalid_role"]).build();
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
        let new_user = UserBuilder::new().with_roles(vec![role]).build();
        let body = create_user(&app, new_user).await;

        if let Some(user_id) = body.get("id").and_then(|id| id.as_str()) {
            delete_user(&app, user_id).await;
        }
    }
}

// ==================== UPDATE USER VALIDATION TESTS ====================

#[tokio::test]
async fn test_update_user_invalid_email() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().build();
    let body = create_user(&app, new_user).await;

    let user_id = body.get("id").and_then(|id| id.as_str()).unwrap();
    let update_user = json!({ "email": "invalid-email-format" });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 400);
    delete_user(&app, user_id).await;
}

#[tokio::test]
async fn test_update_user_password_only_one_field() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().build();

    let body = create_user(&app, new_user).await;
    let user_id = body.get("id").and_then(|id| id.as_str()).unwrap();
    let update_user = json!({ "password": "NewPassword123!" });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 400);
    delete_user(&app, user_id).await;
}

#[tokio::test]
async fn test_update_user_passwords_dont_match() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().build();
    let body = create_user(&app, new_user).await;

    let user_id = body.get("id").and_then(|id| id.as_str()).unwrap();
    let update_user = json!({ "password": "NewPassword123!", "confirmPassword": "DifferentPassword123!" });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 400);
    delete_user(&app, user_id).await;
}

#[tokio::test]
async fn test_update_user_invalid_password_format() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().build();

    let body = create_user(&app, new_user).await;
    let user_id = body.get("id").and_then(|id| id.as_str()).unwrap();

    let update_user = json!({ "password": "Test1!", "confirmPassword": "Test1!" });
    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 400);
    delete_user(&app, user_id).await;
}

#[tokio::test]
async fn test_update_user_invalid_role() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().build();
    let body = create_user(&app, new_user).await;

    let user_id = body.get("id").and_then(|id| id.as_str()).unwrap();
    let update_user = json!({ "roles": ["invalid_role"] });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 400);
    delete_user(&app, user_id).await;
}

#[tokio::test]
async fn test_update_user_valid_password_change() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().build();

    let body = create_user(&app, new_user).await;
    let user_id = body.get("id").and_then(|id| id.as_str()).unwrap();
    let update_user = json!({ "password": "NewValidPassword123!", "confirmPassword": "NewValidPassword123!" });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 200);
    delete_user(&app, user_id).await;
}

#[tokio::test]
async fn test_update_user_valid_role_change() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().build();
    let body = create_user(&app, new_user).await;

    let user_id = body.get("id").and_then(|id| id.as_str()).unwrap();
    let update_user = json!({ "roles": ["teacher", "administrator"] });

    let update_response = app
        .put(&format!("/users/{}", user_id))
        .json(&update_user)
        .await;

    assert_eq!(update_response.status_code(), 200);

    let updated_body = update_response.json::<ResponseBody>();
    let updated_role = updated_body
        .data
        .get("roles")
        .and_then(|roles| roles.as_array())
        .expect("Updated role should be present");

    let roles = updated_role
        .iter()
        .map(|role| role.as_str().unwrap_or(""))
        .collect::<Vec<&str>>();

    assert_eq!(roles, ["teacher", "administrator"]);
    delete_user(&app, user_id).await;
}

// ==================== EDGE CASES AND BOUNDARY TESTS ====================

#[tokio::test]
async fn test_create_user_name_boundary_valid() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new().with_name("Tests").build();
    let body = create_user(&app, new_user).await;

    if let Some(user_id) = body.get("id").and_then(|id| id.as_str()) {
        delete_user(&app, user_id).await;
    }
}

#[tokio::test]
async fn test_create_user_name_boundary_maximum() {
    let app = init_test_app().await;
    let max_name = "a".repeat(100);
    let new_user = UserBuilder::new().with_name(&max_name).build();
    let body = create_user(&app, new_user).await;

    if let Some(user_id) = body.get("id").and_then(|id| id.as_str()) {
        delete_user(&app, user_id).await;
    }
}

#[tokio::test]
async fn test_create_user_password_boundary_minimum() {
    let app = init_test_app().await;
    let new_user = UserBuilder::new()
        .with_password("TestPa1!", "TestPa1!")
        .build();

    let body = create_user(&app, new_user).await;

    if let Some(user_id) = body.get("id").and_then(|id| id.as_str()) {
        delete_user(&app, user_id).await;
    }
}

#[tokio::test]
async fn test_create_user_password_boundary_maximum() {
    let app = init_test_app().await;
    let base_password = "A".repeat(96);
    let password = format!("{}a1!", base_password);
    let new_user = UserBuilder::new()
        .with_password(&password, &password)
        .build();

    let body = create_user(&app, new_user).await;

    if let Some(user_id) = body.get("id").and_then(|id| id.as_str()) {
        delete_user(&app, user_id).await;
    }
}
