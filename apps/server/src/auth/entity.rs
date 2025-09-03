use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user_id: String,
    pub google_access_token: String,
    pub google_refresh_token: String,
}

impl Session {
    pub fn builder() -> SessionBuilder {
        SessionBuilder::new()
    }
}

#[derive(Default)]
pub struct SessionBuilder {
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_in: Option<i64>,
    user_id: Option<String>,
    google_access_token: Option<String>,
    google_refresh_token: Option<String>,
}

impl SessionBuilder {
    pub fn new() -> Self {
        Self {
            access_token: None,
            refresh_token: None,
            expires_in: None,
            user_id: None,
            google_access_token: None,
            google_refresh_token: None,
        }
    }

    pub fn access_token(mut self, token: String) -> Self {
        self.access_token = Some(token);
        self
    }

    pub fn refresh_token(mut self, token: String) -> Self {
        self.refresh_token = Some(token);
        self
    }

    pub fn expires_in(mut self, expires_in: i64) -> Self {
        self.expires_in = Some(expires_in);
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
            access_token: self.access_token.unwrap_or_default(),
            refresh_token: self.refresh_token.unwrap_or_default(),
            expires_in: self.expires_in.unwrap_or(0),
            user_id: self.user_id.unwrap_or_default(),
            google_access_token: self.google_access_token.unwrap_or_default(),
            google_refresh_token: self.google_refresh_token.unwrap_or_default(),
        }
    }
}
