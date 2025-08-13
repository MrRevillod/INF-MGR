use axum_test::TestServer;
use serde_json::{Value, json};
use sword::web::ResponseBody;
use uuid::Uuid;

use crate::extract_resource_id;

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
            rut: generate_unique_rut(),
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

    assert_eq!(
        response.status_code(),
        201,
        "{}",
        format!("Failed to create user - response: {}", body.data)
    );

    body.data
}

pub async fn create_teacher(server: &TestServer, email: Option<String>) -> String {
    let mut user = UserBuilder::new().with_roles(vec!["teacher"]);

    if let Some(email) = email {
        user = user.with_email(&email);
    }

    let data = create_user(server, user.build()).await;

    extract_resource_id(&data)
}

pub async fn create_administrator(server: &TestServer) -> String {
    let user = UserBuilder::new().with_roles(vec!["administrator"]).build();
    let data = create_user(server, user).await;

    extract_resource_id(&data)
}

pub async fn create_student(server: &TestServer, email: Option<String>) -> String {
    let mut user = UserBuilder::new()
        .with_roles(vec!["student"])
        .with_email(&generate_unique_email());

    if let Some(email) = email {
        user = user.with_email(&email);
    }
    let data = create_user(server, user.build()).await;

    extract_resource_id(&data)
}

pub async fn delete_user(server: &TestServer, user_id: &str) {
    let response = server.delete(&format!("/users/{}", user_id)).await;
    assert!(
        response.status_code() == 200 || response.status_code() == 404,
        "Failed to delete user: expected 200 or 404, got {}",
        response.status_code()
    );
}

pub fn generate_unique_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

pub fn generate_unique_rut() -> String {
    let random_number = (Uuid::new_v4().as_u128() % 90000000) + 10000000;
    let verification_digit = calculate_rut_verification_digit(random_number as u32);
    format!("{}-{}", random_number, verification_digit)
}

fn calculate_rut_verification_digit(rut: u32) -> char {
    let sequence = [2, 3, 4, 5, 6, 7];
    let mut sum = 0;
    let mut rut_copy = rut;
    let mut i = 0;

    while rut_copy > 0 {
        sum += (rut_copy % 10) * sequence[i % 6];
        rut_copy /= 10;
        i += 1;
    }

    let remainder = sum % 11;
    match 11 - remainder {
        10 => 'K',
        11 => '0',
        n => char::from_digit(n as u32, 10).unwrap(),
    }
}
