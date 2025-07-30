use crate::{
    inscriptions::infrastructure::InscriptionResponseModel,
    users::{
        application::{
            CreateUserCase, DeleteUserCase, GetUsersCase, UpdateUserCase,
        },
        infrastructure::dtos::{
            CreateUserDto, GetUsersQuery, UpdateUserDto, UserResponseDTO,
        },
    },
};

use crate::shared::di::AppModule;

use serde_json::json;
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

        let data = use_case.get_all(query.try_into()?).await?;
        let users: Vec<UserResponseDTO> =
            data.users.into_iter().map(UserResponseDTO::from).collect();

        let json = json!({
            "users": users,
            "currentPage": data.current_page,
            "totalPages": data.total_pages,
            "totalUsers": data.total_users,
            "hasNext": data.has_next,
            "hasPrevious": data.has_previous,
        });

        Ok(HttpResponse::Ok().data(json))
    }

    #[get("/student/{id}/inscriptions")]
    async fn get_student_inscriptions(ctx: Context) -> HttpResult<HttpResponse> {
        let user_id = ctx.param::<Uuid>("id")?;
        let use_case = ctx.get_dependency::<AppModule, dyn GetUsersCase>()?;

        let user_data = use_case.get_student_inscriptions(&user_id).await?;

        let result = user_data
            .into_iter()
            .map(InscriptionResponseModel::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(result))
    }

    #[get("/{id}")]
    async fn get_user(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let use_case = ctx.get_dependency::<AppModule, dyn GetUsersCase>()?;
        let user = use_case.get_user_by_id(&id).await?;

        let user_response = UserResponseDTO::from(user);

        Ok(HttpResponse::Ok().data(user_response))
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
