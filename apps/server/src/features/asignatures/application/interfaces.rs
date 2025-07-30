use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

use crate::asignatures::domain::{Asignature, AsignatureError, Evaluation};

#[async_trait]
pub trait GetAsignaturesCase: Interface {
    async fn execute(&self) -> Result<Vec<Asignature>, AsignatureError>;
}

#[async_trait]
pub trait CreateAsignatureCase: Interface {
    async fn execute(
        &self,
        input: Asignature,
    ) -> Result<Asignature, AsignatureError>;
}

#[derive(Debug, Clone)]
pub struct UpdateAsignatureInput {
    pub year: Option<i32>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub evaluations: Option<Vec<Evaluation>>,
    pub teacher_id: Option<Uuid>,
    pub coordinator_id: Option<Uuid>,
    pub status: Option<String>,
}

#[async_trait]
pub trait UpdateAsignatureCase: Interface {
    async fn execute(
        &self,
        id: &Uuid,
        input: UpdateAsignatureInput,
    ) -> Result<Asignature, AsignatureError>;
}

#[async_trait]
pub trait DeleteAsignatureCase: Interface {
    async fn execute(&self, id: &Uuid) -> Result<(), AsignatureError>;
}
