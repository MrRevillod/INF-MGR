use crate::{
    auth::{TokenKind, TokenService},
    shared::di::AppModule,
};

use sword::prelude::*;

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

        ctx.extensions.insert(token_claims);

        next!(ctx, next)
    }
}
