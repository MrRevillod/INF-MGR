use crate::users::{
    application::{CreateUserCase, DeleteUserCase, GetUsersCase, UpdateUserCase},
    infrastructure::dtos::{
        CreateUserDto, GetUsersQuery, UpdateUserDto, UserResponseDTO,
    },
};

use crate::shared::di::AppModule;
use sword::{prelude::*, web::HttpResult};
use uuid::Uuid;

#[controller("/users")]
pub struct UserController;

#[routes]
impl UserController {
    #[get("/")]
    async fn get_users(ctx: Context) -> HttpResult<HttpResponse> {
        let query = ctx.validated_query::<GetUsersQuery>()?;
        let use_case = ctx.get_dependency::<AppModule, dyn GetUsersCase>()?;

        let data = use_case.execute(query.role).await?;
        let users: Vec<UserResponseDTO> =
            data.into_iter().map(UserResponseDTO::from).collect();

        Ok(HttpResponse::Ok().data(users))
    }

    #[post("/")]
    async fn create_user(ctx: Context) -> HttpResult<HttpResponse> {
        let use_case = ctx.get_dependency::<AppModule, dyn CreateUserCase>()?;
        let user_data: CreateUserDto = ctx.validated_body()?;

        let user = use_case.execute(user_data.into()).await?;

        Ok(HttpResponse::Created().data(UserResponseDTO::from(user)))
    }

    #[put("/{id}")]
    pub async fn update_user(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let user_data: UpdateUserDto = ctx.validated_body()?;

        let use_case = ctx.get_dependency::<AppModule, dyn UpdateUserCase>()?;
        let user = use_case.execute(&id, user_data.into()).await?;

        Ok(HttpResponse::Ok().data(UserResponseDTO::from(user)))
    }

    #[delete("/{id}")]
    async fn delete_user(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let use_case = ctx.get_dependency::<AppModule, dyn DeleteUserCase>()?;

        use_case.execute(&id).await?;

        Ok(HttpResponse::Ok())
    }
}
