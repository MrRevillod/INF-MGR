use crate::{
    auth::{TokenKind, TokenService},
    shared::di::AppModule,
    users::UserRepository,
};

use sword::prelude::*;
use uuid::Uuid;

pub struct Authentication;

impl Middleware for Authentication {
    async fn handle(mut ctx: Context, next: Next) -> MiddlewareResult {
        let cookies = ctx.cookies()?;

        let Some(access_cookie) = cookies.get("access") else {
            return Err(HttpResponse::Unauthorized());
        };

        let token_service = ctx.di::<AppModule, dyn TokenService>()?;

        let token_claims =
            token_service.verify(TokenKind::Access, access_cookie.value())?;

        let Ok(user_id) = Uuid::parse_str(&token_claims.user_id) else {
            return Err(HttpResponse::Unauthorized());
        };

        let user = ctx
            .di::<AppModule, dyn UserRepository>()?
            .find_by_id(&user_id)
            .await?
            .ok_or(HttpResponse::Unauthorized())?;

        if user.deleted_at.is_some() {
            return Err(HttpResponse::Unauthorized());
        }

        ctx.extensions.insert(user);
        ctx.extensions.insert(token_claims);

        next!(ctx, next)
    }
}
