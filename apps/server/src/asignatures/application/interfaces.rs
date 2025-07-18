use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

use crate::asignatures::{
    application::inputs::{CreateAsignatureInput, UpdateAsignatureInput},
    domain::{Asignature, AsignatureError},
};

#[async_trait]
pub trait GetAsignaturesCase: Interface {
    async fn execute(&self) -> Result<Vec<Asignature>, AsignatureError>;
}

#[async_trait]
pub trait CreateAsignatureCase: Interface {
    async fn execute(
        &self,
        input: CreateAsignatureInput,
    ) -> Result<Asignature, AsignatureError>;
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
