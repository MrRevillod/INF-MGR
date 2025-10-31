use std::sync::Arc;

use crate::auth::{Authentication, MinimumRequiredRole};
use crate::users::*;

use serde_json::json;
use sword::prelude::*;
use uuid::Uuid;

#[controller("/users")]
#[uses(Authentication)]
pub struct UsersController {
    users: Arc<UserService>,
}

#[routes]
impl UsersController {
    #[get("/")]
    #[uses(MinimumRequiredRole, config = Role::Secretary)]
    async fn find_all(&self, req: Request) -> HttpResult {
        let query = req
            .query_validator::<GetUsersQueryDto>()?
            .unwrap_or_default();

        let data = self.users.get_all(query.into()).await?;

        let users = data
            .items
            .into_iter()
            .map(UserResponse::from)
            .collect::<Vec<_>>();

        let json = json!({
            "users": users,
            "currentPage": data.current_page,
            "totalPages": data.total_pages,
            "hasNext": data.has_next,
            "hasPrevious": data.has_previous,
        });

        Ok(HttpResponse::Ok().data(json))
    }

    #[post("/")]
    #[uses(MinimumRequiredRole, config = Role::Secretary)]
    async fn create(&self, req: Request) -> HttpResult {
        let user_data = req.body_validator::<CreateUserDto>()?;
        let user = self.users.create(user_data).await?;

        Ok(HttpResponse::Created().data(UserResponse::from(user)))
    }

    #[patch("/{id}")]
    #[uses(MinimumRequiredRole, config = Role::Secretary)]
    pub async fn update(&self, req: Request) -> HttpResult {
        let id = req.param::<Uuid>("id")?;
        let user_data = req.body_validator::<UpdateUserDto>()?;

        let user = self.users.update(id, user_data).await?;

        Ok(HttpResponse::Ok().data(UserResponse::from(user)))
    }

    #[delete("/{id}")]
    #[uses(MinimumRequiredRole, config = Role::Administrator)]
    async fn remove(&self, req: Request) -> HttpResult {
        let id = req.param::<Uuid>("id")?;

        self.users.remove(id).await?;

        Ok(HttpResponse::Ok())
    }
}
