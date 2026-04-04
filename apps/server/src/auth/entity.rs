use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub access_token: String,
    pub refresh_token: String,

    pub user_id: String,
    pub google_access_token: String,
    pub google_refresh_token: String,

    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn builder() -> SessionBuilder {
        SessionBuilder::new()
    }
}

#[derive(Default)]
pub struct SessionBuilder {
    id: Option<Uuid>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    user_id: Option<String>,
    google_access_token: Option<String>,
    google_refresh_token: Option<String>,
}

impl SessionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn session_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn access_token(mut self, token: String) -> Self {
        self.access_token = Some(token);
        self
    }

    pub fn refresh_token(mut self, token: String) -> Self {
        self.refresh_token = Some(token);
        self
    }

    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn google_access_token(mut self, token: String) -> Self {
        self.google_access_token = Some(token);
        self
    }

    pub fn google_refresh_token(mut self, token: String) -> Self {
        self.google_refresh_token = Some(token);
        self
    }

    pub fn build(self) -> Session {
        Session {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            access_token: self.access_token.unwrap_or_default(),
            refresh_token: self.refresh_token.unwrap_or_default(),
            user_id: self.user_id.unwrap_or_default(),
            google_access_token: self.google_access_token.unwrap_or_default(),
            google_refresh_token: self.google_refresh_token.unwrap_or_default(),
            created_at: Utc::now(),
        }
    }
}
