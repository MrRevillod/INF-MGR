use async_trait::async_trait;
use oauth2::TokenResponse;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    auth::{Session, SessionRepository, TokenKind, TokenService, permissions::*},
    shared::{AppResult, errors::AuthError, oauth::OAuthTokenType},
    users::{User, UserRepository},
};

#[derive(Component)]
#[shaku(interface = SessionService)]
pub struct SessionServiceImpl {
    #[shaku(inject)]
    session_repository: Arc<dyn SessionRepository>,

    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,

    #[shaku(inject)]
    jwt: Arc<dyn TokenService>,
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

        let permissions = Permissions::build(&user.roles);

        let access_token =
            self.jwt
                .sign(TokenKind::Access, &session_id, &permissions, &user_id)?;

        let refresh_token = self.jwt.sign(
            TokenKind::Refresh,
            &session_id,
            &permissions,
            &user_id,
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

        let user = self
            .user_repository
            .find_by_id(&Uuid::parse_str(&claims.user_id).unwrap_or_default())
            .await?
            .ok_or_else(|| AuthError::UserNotFound(claims.user_id.clone()))?;

        let permissions = Permissions::build(&&user.roles);

        let new_access_token = self.jwt.sign(
            TokenKind::Access,
            &claims.session_id,
            &permissions,
            &claims.user_id,
        )?;

        let new_refresh_token = self.jwt.sign(
            TokenKind::Refresh,
            &claims.session_id,
            &permissions,
            &claims.user_id,
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
