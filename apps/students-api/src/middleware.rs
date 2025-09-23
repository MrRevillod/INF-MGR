use crate::ApiConfig;
use sword::prelude::*;

pub struct RequireApiKey;

impl Middleware for RequireApiKey {
    async fn handle(ctx: Context, next: Next) -> MiddlewareResult {
        let Some(key) = ctx.header("x-api-key") else {
            return Err(HttpResponse::Unauthorized().message("Missing API key"))?;
        };

        let api_key = ctx.config::<ApiConfig>()?.api_key;

        if key != api_key {
            return Err(HttpResponse::Unauthorized().message("Invalid API key"))?;
        }

        next!(ctx, next)
    }
}
