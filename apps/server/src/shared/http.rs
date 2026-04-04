use sword::web::{HttpResponse, Request};

use crate::auth::OwnershipValidation;
use crate::shared::RequestFiles;
use crate::users::User;

pub trait ContextExt {
    fn get_ownership_validation(&self) -> Result<OwnershipValidation, HttpResponse>;
    fn get_current_user(&self) -> Result<User, HttpResponse>;
    fn get_bearer_tokens(&self) -> Result<(String, String), HttpResponse>;
    fn files(&self) -> Option<&RequestFiles>;
}

impl ContextExt for Request {
    fn get_ownership_validation(&self) -> Result<OwnershipValidation, HttpResponse> {
        let ownership_validation = self.extensions.get::<OwnershipValidation>();

        let Some(value) = ownership_validation else {
            return Err(HttpResponse::BadRequest().message("Missing required files"));
        };

        Ok(value.clone())
    }

    fn get_current_user(&self) -> Result<User, HttpResponse> {
        let user = self.extensions.get::<User>();

        let Some(user) = user else {
            return Err(
                HttpResponse::InternalServerError().message("User not found in context")
            );
        };

        Ok(user.clone())
    }

    fn get_bearer_tokens(&self) -> Result<(String, String), HttpResponse> {
        let Some(auth_header) = self.header("authorization") else {
            return Err(
                HttpResponse::Unauthorized().message("Authorization header missing")
            );
        };

        if !auth_header.starts_with("Bearer ") {
            return Err(HttpResponse::Unauthorized()
                .message("Invalid authorization header format"));
        }

        let tokens_str = auth_header.trim_start_matches("Bearer ");
        let tokens: Vec<&str> = tokens_str.split(',').collect();

        if tokens.len() != 2 {
            return Err(HttpResponse::Unauthorized().message("Invalid bearer token format. Expected: Bearer access_token,refresh_token"));
        }

        let access_token = tokens[0].trim().to_string();
        let refresh_token = tokens[1].trim().to_string();

        if access_token.is_empty() || refresh_token.is_empty() {
            return Err(HttpResponse::Unauthorized()
                .message("Access token or refresh token is empty"));
        }

        Ok((access_token, refresh_token))
    }

    fn files(&self) -> Option<&RequestFiles> {
        self.extensions.get::<RequestFiles>()
    }
}
