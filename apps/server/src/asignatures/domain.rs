use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Evaluation {
    pub id: Uuid,
    pub name: String,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct Asignature {
    pub id: Uuid,
    pub year: i32,
    pub code: String,
    pub name: String,
    pub evaluations: Vec<Evaluation>,
    pub teacher_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct AsignatureFilter {
    pub year: Option<i32>,
    pub code: Option<String>,
    pub name: Option<String>,
}

#[async_trait]
pub trait AsignatureRepository: Interface {
    async fn find_all(&self) -> Result<Vec<Asignature>, AsignatureError>;
    async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<Asignature>, AsignatureError>;

    async fn find_by_filter(
        &self,
        filter: AsignatureFilter,
    ) -> Result<Vec<Asignature>, AsignatureError>;

    async fn create(&self, input: Asignature)
        -> Result<Asignature, AsignatureError>;

    async fn update(
        &self,
        id: &Uuid,
        input: Asignature,
    ) -> Result<Asignature, AsignatureError>;

    async fn delete(&self, id: &Uuid) -> Result<(), AsignatureError>;
}

#[derive(Debug)]
pub enum AsignatureError {
    NotFound,
    AlreadyExists,
    UnexpectedError(String),
    InvalidIdentifier,
    DatabaseError(String),
    UserIsNotTeacher,
    TeacherNotFound,
}
