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

        if !user_permissions.contains(&required_permission.to_string()) {
            tracing::warn!(
                "Permission denied. Required: {}, User permissions: {:?}",
                required_permission,
                user_permissions
            );

            return Err(
                HttpResponse::Forbidden().message("Missing required permission")
            );
        }

        next!(req, next)
    }
}
