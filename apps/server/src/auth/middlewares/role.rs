use std::str::FromStr;
use sword::prelude::*;
use uuid::Uuid;

use crate::users::{Role, User};

pub struct MinimumRequiredRole;

#[derive(Debug, Clone)]
pub struct OwnershipValidation {
    pub required: bool,
    pub user_id: Uuid,
}

impl MiddlewareWithConfig<&str> for MinimumRequiredRole {
    async fn handle(role: &str, mut ctx: Context, next: Next) -> MiddlewareResult {
        let minimum_required_role =
            Role::from_str(role).map_err(|_| HttpResponse::BadRequest())?;

        let Some(user) = ctx.extensions.get::<User>() else {
            return Err(HttpResponse::Unauthorized());
        };

        let user_role_priority = user.role.priority();

        let has_required_role =
            user_role_priority >= minimum_required_role.priority();

        if !has_required_role {
            return Err(HttpResponse::Forbidden().message("Insufficient role"));
        }

        // Determinar si necesita validación de ownership basado en la prioridad
        // Prioridades altas (Admin=4, Secretary=3) no necesitan validación
        // Prioridades bajas (Teacher=2, Student=1) sí necesitan validación
        let ownership_validation = OwnershipValidation {
            required: user_role_priority < 3, // Solo Teacher(2) y Student(1) necesitan validación
            user_id: user.id,
        };

        ctx.extensions.insert(ownership_validation);

        next!(ctx, next)
    }
}
