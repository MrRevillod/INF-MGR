use async_trait::async_trait;
use oauth2::{CsrfToken, Scope};
use shaku::{Component, Interface};
use std::sync::Arc;

use crate::{auth::repository::AuthRepository, shared::oauth::OAuthClient};

#[derive(Component)]
#[shaku(interface = OAuthService)]
pub struct GoogleOAuthService {
    #[shaku(inject)]
    oauth_client: Arc<dyn OAuthClient>,

    #[shaku(inject)]
    auth_repository: Arc<dyn AuthRepository>,
}

#[async_trait]
pub trait OAuthService: Interface {
    async fn get_login_url(&self) -> (String, String);
}

#[async_trait]
impl OAuthService for GoogleOAuthService {
    async fn get_login_url(&self) -> (String, String) {
        let client = self.oauth_client.get_client();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("email".into()))
            .add_scope(Scope::new("profile".into()))
            .add_scope(Scope::new("profile".into()))
            .url();

        unimplemented!()
    }
}
