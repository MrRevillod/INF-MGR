use axum_test::TestServer;
use serde_json::{json, Value};
use sword::web::ResponseBody;
use uuid::Uuid;

pub struct UserBuilder {
    rut: String,
    name: String,
    email: String,
    roles: Vec<String>,
    password: String,
    confirm_password: String,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            rut: "34108499-7".to_string(),
            name: "Test User".to_string(),
            email: generate_unique_email(),
            roles: vec!["administrator".to_string()],
            password: "TestPassword123!".to_string(),
            confirm_password: "TestPassword123!".to_string(),
        }
    }

    pub fn with_rut(mut self, rut: &str) -> Self {
        self.rut = rut.to_string();
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_email(mut self, email: &str) -> Self {
        self.email = email.to_string();
        self
    }

    pub fn with_roles(mut self, roles: Vec<&str>) -> Self {
        self.roles = roles.iter().map(|r| r.to_string()).collect();
        self
    }

    pub fn with_password(mut self, password: &str, confirm_password: &str) -> Self {
        self.password = password.to_string();
        self.confirm_password = confirm_password.to_string();
        self
    }

    pub fn build(self) -> Value {
        json!({
            "rut": self.rut,
            "name": self.name,
            "email": self.email,
            "roles": self.roles,
            "password": self.password,
            "confirmPassword": self.confirm_password,
        })
    }
}

pub async fn create_user(server: &TestServer, user: Value) -> Value {
    let response = server.post("/users").json(&user).await;
    let body = response.json::<ResponseBody>();

    assert_eq!(response.status_code(), 201, "Failed to create user");

    body.data
}

pub async fn delete_user(server: &TestServer, user_id: &str) {
    let response = server.delete(&format!("/users/{}", user_id)).await;
    // Accept both 200 (successful deletion) and 404 (user not found/already deleted)
    assert!(
        response.status_code() == 200 || response.status_code() == 404,
        "Failed to delete user: expected 200 or 404, got {}",
        response.status_code()
    );
}

pub fn generate_unique_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
