use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

use crate::{
    auth::{Session, SessionRepository},
    shared::{AppResult, oauth::OAuthTokenType},
    users::User,
};

#[derive(Component)]
#[shaku(interface = SessionService)]
pub struct SessionServiceImpl {
    #[shaku(inject)]
    _repository: Arc<dyn SessionRepository>,
}

#[async_trait]
pub trait SessionService: Interface {
    async fn create_session(&self, user: &User, oauth_token: OAuthTokenType) -> AppResult<Session>;
}

#[async_trait]
impl SessionService for SessionServiceImpl {
    async fn create_session(
        &self,
        _user: &User,
        _oauth_token: OAuthTokenType,
    ) -> AppResult<Session> {
        unimplemented!()
    }
}
