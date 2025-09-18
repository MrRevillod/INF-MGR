use sword::prelude::*;

use crate::{
    auth::permissions::Permission,
    shared::{AppError, errors::AuthError},
};

pub struct RequirePermission {}

impl MiddlewareWithConfig<&str> for RequirePermission {
    async fn handle(
        permission: &str,
        mut ctx: Context,
        next: Next,
    ) -> MiddlewareResult {
        let required_permission =
            Permission::try_from(permission).map_err(AppError::from)?;

        let user_permissions: Vec<Permission> = ctx
            .extensions
            .get::<Vec<String>>()
            .ok_or(AppError::from(AuthError::Other(
                "No permissions found".to_string(),
            )))?
            .iter()
            .filter_map(|p| Permission::try_from(p.as_str()).ok())
            .collect();

        let valid_permission = user_permissions
            .iter()
            .find(|p| p.matches_without_scope(&required_permission))
            .ok_or(AppError::from(AuthError::Other(format!(
                "Missing required permission: {}:{}",
                required_permission.resource, required_permission.action
            ))))?;

        ctx.extensions.remove::<Vec<String>>();
        ctx.extensions
            .insert::<Permission>(valid_permission.clone());

        next!(ctx, next)
    }
}
