use crate::container::AppModule;
use crate::users::{CreateUserDto, GetUsersQueryDto, UpdateUserDto, UserResponse, UserService};

use serde_json::json;
use sword::{prelude::*, web::HttpResult};
use uuid::Uuid;

#[controller("/users")]
pub struct UsersController;

#[routes]
impl UsersController {
    #[get("/")]
    async fn find_all(ctx: Context) -> HttpResult<HttpResponse> {
        let query = ctx.validated_query::<GetUsersQueryDto>()?;
        let service = ctx.get_dependency::<AppModule, dyn UserService>()?;

        let data = service.get_all(query.into()).await?;
        let users = data.items.into_iter().map(UserResponse::from).collect::<Vec<_>>();

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
    async fn create(ctx: Context) -> HttpResult<HttpResponse> {
        let user_data = ctx.validated_body::<CreateUserDto>()?;
        let service = ctx.get_dependency::<AppModule, dyn UserService>()?;

        let user = service.create(user_data).await?;

        Ok(HttpResponse::Created().data(UserResponse::from(user)))
    }

    #[patch("/{id}")]
    pub async fn update(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let user_data = ctx.validated_body::<UpdateUserDto>()?;

        let service = ctx.get_dependency::<AppModule, dyn UserService>()?;
        let user = service.update(id, user_data).await?;

        Ok(HttpResponse::Ok().data(UserResponse::from(user)))
    }

    #[delete("/{id}")]
    async fn remove(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let service = ctx.get_dependency::<AppModule, dyn UserService>()?;

        service.remove(id).await?;

        Ok(HttpResponse::Ok())
    }
}
