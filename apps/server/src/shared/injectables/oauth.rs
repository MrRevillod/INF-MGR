use crate::config::AuthConfig;
use client::OAuthClientType;

use oauth2::{
    AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl, basic::BasicClient,
};

use reqwest::{Client as HttpClient, ClientBuilder, redirect::Policy};

pub use client::OAuthTokenType;
use sword::core::injectable;

#[injectable(provider)]
pub struct OAuthClient {
    client: OAuthClientType,
    http_client: HttpClient,
}

impl OAuthClient {
    pub fn new(config: &AuthConfig) -> Self {
        let client_id = ClientId::new(config.google_client_id.clone());
        let client_secret = ClientSecret::new(config.google_client_secret.clone());
        let auth_url =
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                .expect("Invalid authorization endpoint URL");

        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .expect("Invalid token endpoint URL");

        let redirect_url = RedirectUrl::new(config.google_redirect_url.clone())
            .expect("Invalid redirect URL");

        let oauth_client = BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url);

        let http_client = ClientBuilder::new()
            .redirect(Policy::none())
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client: oauth_client,
            http_client,
        }
    }

    pub const fn get_client(&self) -> &OAuthClientType {
        &self.client
    }

    pub const fn get_http_client(&self) -> &HttpClient {
        &self.http_client
    }
}

mod client {

    use oauth2::{
        Client, EmptyExtraTokenFields, EndpointNotSet, EndpointSet,
        RevocationErrorResponseType, StandardErrorResponse, StandardRevocableToken,
        StandardTokenIntrospectionResponse, StandardTokenResponse,
        basic::{BasicErrorResponseType, BasicTokenType},
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

    pub type OAuthTokenType =
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
}
