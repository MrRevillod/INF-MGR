use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub rut: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}
