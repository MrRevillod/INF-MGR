use async_trait::async_trait;
use oauth2::TokenResponse;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    auth::*,
    shared::{AppResult, AuthError, OAuthTokenType},
    users::User,
};

#[derive(Component)]
#[shaku(interface = SessionService)]
pub struct SessionServiceImpl {
    #[shaku(inject)]
    session_repository: Arc<dyn SessionRepository>,

    #[shaku(inject)]
    jwt: Arc<dyn TokenService>,

    #[shaku(inject)]
    oauth_service: Arc<dyn OAuthService>,
}

#[async_trait]
pub trait SessionService: Interface {
    async fn create_session(
        &self,
        user: &User,
        oauth_token: OAuthTokenType,
        ttl: u64,
    ) -> AppResult<Session>;

    async fn refresh_session(
        &self,
        refresh_token: String,
        ttl: u64,
    ) -> AppResult<Session>;

    async fn close_session(&self, access_token: &str) -> AppResult<()>;
}

#[async_trait]
impl SessionService for SessionServiceImpl {
    async fn create_session(
        &self,
        user: &User,
        oauth_token: OAuthTokenType,
        ttl: u64,
    ) -> AppResult<Session> {
        let user_id = user.id.to_string();
        let session_id = Uuid::new_v4().to_string();

        let access_token = self.jwt.sign(
            TokenKind::Access,
            &session_id,
            &user_id,
            &user.role.to_string(),
        )?;

        let refresh_token = self.jwt.sign(
            TokenKind::Refresh,
            &session_id,
            &user_id,
            &user.role.to_string(),
        )?;

        let (oauth_access_token, oauth_refresh_token) = (
            oauth_token.access_token().secret().clone(),
            oauth_token
                .refresh_token()
                .map(|t| t.secret().clone())
                .unwrap_or_default(),
        );

        let session = Session::builder()
            .user_id(user_id)
            .access_token(access_token)
            .refresh_token(refresh_token)
            .google_access_token(oauth_access_token)
            .google_refresh_token(oauth_refresh_token)
            .build();

        self.session_repository.save(session.clone(), ttl).await?;

        Ok(session)
    }

    async fn refresh_session(
        &self,
        refresh_token: String,
        ttl: u64,
    ) -> AppResult<Session> {
        let claims = self.jwt.verify(TokenKind::Refresh, &refresh_token)?;

        let mut session = self
            .session_repository
            .get(&claims.session_id)
            .await?
            .ok_or(AuthError::SessionExpired)?;

        if session.refresh_token != refresh_token {
            return Err(AuthError::SessionExpired.into());
        }

        if let Some((new_google_access, new_google_refresh)) =
            self.try_refresh_google_tokens(&session).await
        {
            session.google_access_token = new_google_access;
            session.google_refresh_token = new_google_refresh;
        }

        let new_access_token = self.jwt.sign(
            TokenKind::Access,
            &claims.session_id,
            &claims.user_id,
            &claims.role,
        )?;

        let new_refresh_token = self.jwt.sign(
            TokenKind::Refresh,
            &claims.session_id,
            &claims.user_id,
            &claims.role,
        )?;

        session.access_token = new_access_token;
        session.refresh_token = new_refresh_token;

        self.session_repository.save(session.clone(), ttl).await?;

        Ok(session)
    }

    async fn close_session(&self, access_token: &str) -> AppResult<()> {
        let claims = self.jwt.verify(TokenKind::Access, access_token)?;
        self.session_repository.delete(&claims.session_id).await?;

        Ok(())
    }
}

impl SessionServiceImpl {
    async fn try_refresh_google_tokens(
        &self,
        session: &Session,
    ) -> Option<(String, String)> {
        if session.google_refresh_token.is_empty() {
            return None;
        }

        let token_response = self
            .oauth_service
            .refresh_oauth_token(&session.google_refresh_token)
            .await
            .ok()?;

        let new_access_token = token_response.access_token().secret().clone();
        let new_refresh_token = token_response
            .refresh_token()
            .map(|t| t.secret().clone())
            .unwrap_or_else(|| session.google_refresh_token.clone());

        Some((new_access_token, new_refresh_token))
    }
}
