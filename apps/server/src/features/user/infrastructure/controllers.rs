use axum::{extract::Path, http::StatusCode};
use axum_responses::http::{ControllerResult, HttpResponse};
use serde_json::json;

use crate::{
    features::user::{
        application::interfaces::{
            CreateUserCase, DeleteUserCase, GetUsersCase, UpdateUserCase,
        },
        infrastructure::dtos::{CreateUserDto, UpdateUserDto},
    },
    shared::infrastructure::{extractors::BodyValidator, Inject},
};

use super::models::{UserModel, UserResponseDTO};

pub async fn get_users(use_case: Inject<dyn GetUsersCase>) -> ControllerResult {
    let data = use_case.execute().await?;
    let users: Vec<UserResponseDTO> =
        data.into_iter().map(UserResponseDTO::from).collect();

    HttpResponse::build()
        .code(200)
        .body(json!({ "data": users }))
        .wrap()
}

pub async fn create_user(
    use_case: Inject<dyn CreateUserCase>,
    BodyValidator(user_data): BodyValidator<CreateUserDto>,
) -> ControllerResult {
    let user = use_case.execute(user_data.into()).await?;

    HttpResponse::build()
        .status(StatusCode::CREATED)
        .body(json!({ "data": UserModel::from(user) }))
        .wrap()
}

pub async fn update_user(
    use_case: Inject<dyn UpdateUserCase>,
    Path(id): Path<String>,
    BodyValidator(user_data): BodyValidator<UpdateUserDto>,
) -> ControllerResult {
    let user = use_case.execute(id, user_data.into()).await?;
    HttpResponse::build()
        .status(StatusCode::OK)
        .body(json!({ "data": UserModel::from(user) }))
        .wrap()
}

pub async fn delete_user(
    use_case: Inject<dyn DeleteUserCase>,
    Path(id): Path<String>,
) -> ControllerResult {
    use_case.execute(id).await?;
    HttpResponse::build()
        .status(StatusCode::OK)
        .body(json!({ "message": "User deleted successfully" }))
        .wrap()
}
