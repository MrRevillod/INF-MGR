use client::OAuthClientType;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use shaku::{Component, Interface};

use crate::config::AuthConfig;

pub trait OAuthClient: Interface {
    fn get_client(&self) -> &OAuthClientType;
}

#[derive(Component)]
#[shaku(interface = OAuthClient)]
pub struct GoogleOAuthClient {
    client: OAuthClientType,
}

impl GoogleOAuthClient {
    pub fn new(config: &AuthConfig) -> Self {
        let client_id = ClientId::new(config.google_client_id.clone());
        let client_secret = ClientSecret::new(config.google_client_secret.clone());
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");

        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .expect("Invalid token endpoint URL");

        let redirect_url =
            RedirectUrl::new(config.google_redirect_url.clone()).expect("Invalid redirect URL");

        let client = BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url);

        Self { client }
    }
}

impl OAuthClient for GoogleOAuthClient {
    fn get_client(&self) -> &OAuthClientType {
        &self.client
    }
}

mod client {

    use oauth2::{
        basic::{BasicErrorResponseType, BasicTokenType},
        Client, EmptyExtraTokenFields, EndpointNotSet, EndpointSet, RevocationErrorResponseType,
        StandardErrorResponse, StandardRevocableToken, StandardTokenIntrospectionResponse,
        StandardTokenResponse,
    };

    pub type OAuthClientType = Client<
        StandardErrorResponse<BasicErrorResponseType>,
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardRevocableToken,
        StandardErrorResponse<RevocationErrorResponseType>,
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >;
}
