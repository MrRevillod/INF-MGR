use crate::{
    auth::{JsonWebTokenService, TokenKind},
    shared::http::ContextExt,
    types::{Arc, Uuid},
    users::UserRepository,
};

use sword::prelude::*;

#[middleware]
pub struct Authentication {
    jwt: Arc<JsonWebTokenService>,
    user_repository: Arc<UserRepository>,
}

impl OnRequest for Authentication {
    async fn on_request(&self, mut req: Request) -> MiddlewareResult {
        let (access_token, _) = req.get_bearer_tokens()?;

        let token_claims = self.jwt.verify(TokenKind::Access, &access_token)?;

        let Ok(user_id) = Uuid::parse_str(&token_claims.user_id) else {
            return Err(HttpResponse::Unauthorized());
        };

        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(HttpResponse::Unauthorized)?;

        if user.deleted_at.is_some() {
            return Err(HttpResponse::Unauthorized());
        }

        req.extensions.insert(user);
        req.extensions.insert(token_claims);

        req.next().await
    }
}
