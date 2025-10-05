use sword::__internal::IntoResponse;
use sword::web::{Context, HttpResponse, HttpResult, StatusCode};

use crate::auth::OwnershipValidation;
use crate::users::User;

pub trait ContextExt {
    fn get_ownership_validation(&self) -> Result<OwnershipValidation, HttpResponse>;
    fn get_current_user(&self) -> HttpResult<User>;
}

impl ContextExt for Context {
    fn get_ownership_validation(&self) -> Result<OwnershipValidation, HttpResponse> {
        let ownership_validation = self.extensions.get::<OwnershipValidation>();

        let Some(value) = ownership_validation else {
            return Err(HttpResponse::InternalServerError()
                .message("OwnershipValidation not found in context"));
        };

        Ok(value.clone())
    }

    fn get_current_user(&self) -> HttpResult<User> {
        let user = self.extensions.get::<User>();

        let Some(user) = user else {
            return Err(HttpResponse::InternalServerError()
                .message("User not found in context"));
        };

        Ok(user.clone())
    }
}

pub enum FileResponse {
    Document(Vec<u8>),
}

impl IntoResponse for FileResponse {
    fn into_response(self) -> sword::__internal::AxumResponse {
        match self {
            FileResponse::Document(data) => {
                (StatusCode::OK, [("Content-type", "application/pdf")], data)
                    .into_response()
            }
        }
    }
}
