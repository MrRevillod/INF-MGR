use sword::prelude::*;
use uuid::Uuid;

use crate::users::{Role, User};

#[derive(Debug, Clone)]
pub struct OwnershipValidation {
    pub required: bool,
    pub user_id: Uuid,
    pub user_role: Role,
}

#[middleware]
pub struct MinimumRequiredRole {}

impl OnRequestWithConfig<Role> for MinimumRequiredRole {
    async fn on_request_with_config(
        &self,
        role: Role,
        mut req: Request,
    ) -> MiddlewareResult {
        let Some(user) = req.extensions.get::<User>() else {
            return Err(HttpResponse::Unauthorized());
        };

        let user_role_priority = user.role.priority();
        let has_required_role = user_role_priority >= role.priority();

        if !has_required_role {
            return Err(HttpResponse::Unauthorized());
        }

        // Determinar si necesita validación de ownership basado en la prioridad
        // Prioridades altas (Admin=4, Secretary=3) no necesitan validación
        // Prioridades bajas (Teacher=2, Student=1) sí necesitan validación
        let ownership_validation = OwnershipValidation {
            required: user_role_priority < 3, // Solo Teacher(2) y Student(1) necesitan validación
            user_id: user.id,
            user_role: user.role.clone(),
        };

        req.extensions.insert(ownership_validation);

        req.next().await
    }
}
