use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum NotFoundError {
    #[error("User with id {id} not found")]
    User { id: Uuid },

    #[error("Course with id {id} not found")]
    Course { id: Uuid },

    #[error("Enrollment with id {id} not found")]
    Enrollment { id: Uuid },

    #[error("Practice with id {id} not found")]
    Practice { id: Uuid },
}

impl NotFoundError {
    pub fn user(id: Uuid) -> Self {
        Self::User { id }
    }

    pub fn course(id: Uuid) -> Self {
        Self::Course { id }
    }

    pub fn enrollment(id: Uuid) -> Self {
        Self::Enrollment { id }
    }

    pub fn practice(id: Uuid) -> Self {
        Self::Practice { id }
    }
}
