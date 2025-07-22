use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shaku::Interface;
use uuid::Uuid;

use crate::practices::domain::{Practice, PracticeError};

#[async_trait]
pub trait GetPracticeCase: Interface {
    async fn execute(&self, id: &Uuid) -> Result<Practice, PracticeError>;
}

#[async_trait]
pub trait CreatePracticeCase: Interface {
    async fn execute(&self, input: Practice) -> Result<Practice, PracticeError>;
}

pub struct UpdatePracticeInput {
    pub enterprise_name: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub supervisor_name: Option<String>,
    pub supervisor_email: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait UpdatePracticeCase: Interface {
    async fn execute(
        &self,
        id: &Uuid,
        input: UpdatePracticeInput,
    ) -> Result<Practice, PracticeError>;
}

#[async_trait]
pub trait DeletePracticeCase: Interface {
    async fn execute(&self, id: &Uuid) -> Result<(), PracticeError>;
}
