use sword::__internal::IntoResponse;
use sword::web::{Context, HttpResponse, HttpResult, StatusCode};

use crate::auth::OwnershipValidation;
use crate::users::User;

pub trait ContextExt {
    fn get_ownership_validation(&self) -> Result<OwnershipValidation, HttpResponse>;
    fn get_current_user(&self) -> HttpResult<User>;
    fn get_bearer_tokens(&self) -> HttpResult<(String, String)>;
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

    fn get_bearer_tokens(&self) -> HttpResult<(String, String)> {
        // Get tokens from Authorization header in format: Bearer access_token,refresh_token
        let Some(auth_header) = self.header("authorization") else {
            return Err(HttpResponse::Unauthorized().message("Authorization header missing"));
        };

        if !auth_header.starts_with("Bearer ") {
            return Err(HttpResponse::Unauthorized().message("Invalid authorization header format"));
        }

        let tokens_str = auth_header.trim_start_matches("Bearer ");
        let tokens: Vec<&str> = tokens_str.split(',').collect();
        
        if tokens.len() != 2 {
            return Err(HttpResponse::Unauthorized().message("Invalid bearer token format. Expected: Bearer access_token,refresh_token"));
        }

        let access_token = tokens[0].trim().to_string();
        let refresh_token = tokens[1].trim().to_string();

        if access_token.is_empty() || refresh_token.is_empty() {
            return Err(HttpResponse::Unauthorized().message("Access token or refresh token is empty"));
        }

        Ok((access_token, refresh_token))
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
