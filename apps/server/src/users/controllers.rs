use crate::auth::Authentication;
use crate::auth::permissions::RequirePermission;
use crate::shared::di::AppModule;
use crate::users::{
    CreateUserDto, GetUsersQueryDto, UpdateUserDto, UserResponse, UserService,
};

use serde_json::json;
use sword::prelude::*;
use uuid::Uuid;

#[controller("/users")]
pub struct UsersController;

#[routes]
impl UsersController {
    #[get("/")]
    #[middleware(Authentication)]
    #[middleware(RequirePermission, config = "users:read")]
    async fn find_all(ctx: Context) -> HttpResult<HttpResponse> {
        let query = ctx
            .validated_query::<GetUsersQueryDto>()?
            .unwrap_or_default();

        let service = ctx.di::<AppModule, dyn UserService>()?;

        let data = service.get_all(query.into()).await?;
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
    #[middleware(Authentication)]
    #[middleware(RequirePermission, config = "users:create")]
    async fn create(ctx: Context) -> HttpResult<HttpResponse> {
        let user_data = ctx.validated_body::<CreateUserDto>()?;
        let service = ctx.di::<AppModule, dyn UserService>()?;

        let user = service.create(user_data).await?;

        Ok(HttpResponse::Created().data(UserResponse::from(user)))
    }

    #[patch("/{id}")]
    #[middleware(Authentication)]
    #[middleware(RequirePermission, config = "users:update")]
    pub async fn update(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let user_data = ctx.validated_body::<UpdateUserDto>()?;

        let service = ctx.di::<AppModule, dyn UserService>()?;
        let user = service.update(id, user_data).await?;

        Ok(HttpResponse::Ok().data(UserResponse::from(user)))
    }

    #[delete("/{id}")]
    #[middleware(Authentication)]
    #[middleware(RequirePermission, config = "users:delete")]
    async fn remove(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let service = ctx.di::<AppModule, dyn UserService>()?;

        service.remove(id).await?;

        Ok(HttpResponse::Ok())
    }
}
