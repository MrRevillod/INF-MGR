use sword::prelude::*;

use crate::{
    auth::permissions::Permission,
    shared::{AppError, errors::AuthError},
};

pub struct RequirePermission {}

impl MiddlewareWithConfig<&str> for RequirePermission {
    async fn handle(permission: &str, req: Context, next: Next) -> MiddlewareResult {
        let required_permission =
            Permission::try_from(permission).map_err(AppError::from)?;

        let user_permissions = req.extensions.get::<Vec<String>>().ok_or(
            AppError::from(AuthError::Other(
                "No permissions found in request context".to_string(),
            )),
        )?;

        todo!(
            "check just first 2 levels of permission hierarchy and save permissions as enums on the request extensions"
        );

        next!(req, next)
    }
}
