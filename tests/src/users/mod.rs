pub mod utils;

use crate::{app::init_test_app, extract_resource_id};
use serde_json::json;
use utils::*;

#[tokio::test]
async fn test_create_user_should_work() -> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;
    let new_user = TestUser::builder().build();
    let body = TestUser::create_user(&app, new_user).await;

    let user_id = extract_resource_id(&body);

    TestUser::delete(&app, &user_id).await;

    Ok(())
}

#[tokio::test]
async fn test_get_users() -> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;
    let response = app.get("/users?role=students").await;

    assert_eq!(response.status_code(), 200);

    Ok(())
}

#[tokio::test]
async fn just_test_update_user() -> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let new_user = TestUser::builder().build();
    let body = TestUser::create_user(&app, new_user).await;

    let user_id = extract_resource_id(&body);
    let new_email = TestUser::generate_unique_email();
    let update_user = json!({ "email": new_email });

    let updated_data = TestUser::update(&app, &user_id, update_user).await;

    let updated_user_email = updated_data
        .get("email")
        .and_then(|email| email.as_str())
        .expect("Updated user email should be present");

    assert_eq!(updated_user_email, new_email);

    TestUser::delete(&app, &user_id).await;

    Ok(())
}

// ==================== CREATE USER VALIDATION TESTS ====================

#[tokio::test]
async fn test_create_user_invalid_rut_format()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let new_user = TestUser::builder().with_rut("34108499").build();
    let response = TestUser::create_user_no_extract(&app, new_user).await;

    assert_eq!(response.code, 400_u16);

    Ok(())
}

#[tokio::test]
async fn test_create_user_invalid_rut_verification_digit()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;
    let new_user = TestUser::builder().with_rut("34108499-9").build();
    let response = TestUser::create_user_no_extract(&app, new_user).await;

    assert_eq!(response.code, 400_u16);

    Ok(())
}

#[tokio::test]
async fn test_create_user_name_too_short() -> Result<(), Box<dyn std::error::Error>>
{
    let app = init_test_app().await?;

    let new_user = TestUser::builder().with_name("Test").build();
    let response = TestUser::create_user_no_extract(&app, new_user).await;

    assert_eq!(response.code, 400_u16);

    Ok(())
}

#[tokio::test]
async fn test_create_user_name_too_long() -> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;
    let long_name = "a".repeat(101);

    let new_user = TestUser::builder().with_name(&long_name).build();
    let response = TestUser::create_user_no_extract(&app, new_user).await;

    assert_eq!(response.code, 400_u16);

    Ok(())
}

#[tokio::test]
async fn test_create_user_invalid_email() -> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let new_user = TestUser::builder().with_email("invalid-email").build();
    let response = TestUser::create_user_no_extract(&app, new_user).await;

    assert_eq!(response.code, 400_u16);

    Ok(())
}

#[tokio::test]
async fn test_create_user_invalid_role() -> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;

    let new_user = TestUser::builder().with_role("invalid_role").build();
    let response = TestUser::create_user_no_extract(&app, new_user).await;

    assert_eq!(response.code, 400);

    Ok(())
}

#[tokio::test]
async fn test_create_user_valid_roles() -> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;
    let valid_roles = vec!["administrator", "student", "teacher", "secretary"];

    for role in valid_roles {
        let new_user = TestUser::builder().with_role(role).build();
        let body = TestUser::create_user(&app, new_user).await;

        let user_id = extract_resource_id(&body);

        TestUser::delete(&app, &user_id).await;
    }

    Ok(())
}

// ==================== UPDATE USER VALIDATION TESTS ====================

#[tokio::test]
async fn test_update_user_invalid_email() -> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;
    let new_user = TestUser::builder().build();
    let body = TestUser::create_user(&app, new_user).await;

    let user_id = extract_resource_id(&body);
    let update_user = json!({ "email": "invalid-email-format" });

    let updated_response =
        TestUser::update_no_extract(&app, &user_id, update_user).await;

    assert_eq!(updated_response.code, 400);

    TestUser::delete(&app, &user_id).await;

    Ok(())
}

#[tokio::test]
async fn test_update_user_invalid_role() -> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;
    let new_user = TestUser::builder().build();
    let body = TestUser::create_user(&app, new_user).await;

    let user_id = body.get("id").and_then(|id| id.as_str()).unwrap();
    let update_user = json!({ "role": "invalid_role" });

    let updated_response =
        TestUser::update_no_extract(&app, &user_id, update_user).await;

    assert_eq!(updated_response.code, 400);
    TestUser::delete(&app, user_id).await;

    Ok(())
}

#[tokio::test]
async fn test_update_user_valid_role_change()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;
    let new_user = TestUser::builder().build();
    let body = TestUser::create_user(&app, new_user).await;

    let user_id = extract_resource_id(&body);
    let update_user = json!({ "role": "teacher" });

    let updated_response =
        TestUser::update_no_extract(&app, &user_id, update_user).await;

    assert_eq!(updated_response.code, 200);

    let updated_role = updated_response
        .data
        .get("role")
        .and_then(|role| role.as_str())
        .expect("Updated role should be present");

    assert_eq!(updated_role, "teacher");

    TestUser::delete(&app, &user_id).await;

    Ok(())
}

// ==================== EDGE CASES AND BOUNDARY TESTS ====================

#[tokio::test]
async fn test_create_user_name_boundary_valid()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;
    let new_user = TestUser::builder().with_name("Tests").build();
    let body = TestUser::create_user(&app, new_user).await;

    if let Some(user_id) = body.get("id").and_then(|id| id.as_str()) {
        TestUser::delete(&app, user_id).await;
    }

    Ok(())
}

#[tokio::test]
async fn test_create_user_name_boundary_maximum()
-> Result<(), Box<dyn std::error::Error>> {
    let app = init_test_app().await?;
    let max_name = "a".repeat(100);
    let new_user = TestUser::builder().with_name(&max_name).build();
    let body = TestUser::create_user(&app, new_user).await;

    if let Some(user_id) = body.get("id").and_then(|id| id.as_str()) {
        TestUser::delete(&app, user_id).await;
    }

    Ok(())
}
