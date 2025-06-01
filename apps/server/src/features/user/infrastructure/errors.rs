// This file implements the conversion from the `UserError` enum
// to the `HttpResponse` type.

// This is necessary bc the `UserError` enum is used in the internal
// application and domain layers

// |--------------------------------------------|----------------|
// |  User Infrastructure Layer (HttpResponse)  |   Controller   |
// |--------------------------------------------|----------------|
// |     User Application Layer (UserError)     |    Use Case    |
// |--------------------------------------------|----------------|
// |       User Domain Layer (UserError)        |   Repository   |
// |--------------------------------------------|----------------|

use axum::http::StatusCode;
use axum_responses::http::HttpResponse;
use serde_json::json;

use crate::features::user::domain::UserError;

// Each variant of the `UserError` enum corresponds to a specific error
// that can occur in the user management process.

impl From<UserError> for HttpResponse {
    fn from(value: UserError) -> Self {
        match value {
            UserError::UsernameAlreadyExists => HttpResponse::build()
                .status(StatusCode::CONFLICT)
                .body(json!({
                    "field": "username",
                    "message": "Username already exists",
                })),

            UserError::EmailAlreadyExists => HttpResponse::build()
                .status(StatusCode::CONFLICT)
                .body(json!({
                    "field": "email",
                    "message": "Email already exists",
                })),

            UserError::NotFound => HttpResponse::build()
                .status(StatusCode::NOT_FOUND)
                .body(json!({
                    "message": "User not found",
                })),

            UserError::UnexpectedError => HttpResponse::build()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(json!({
                    "message": "Unexpected error",
                })),

            UserError::InvalidEmail => HttpResponse::build()
                .status(StatusCode::BAD_REQUEST)
                .body(json!({
                    "field": "email",
                    "message": "The provided email is not valid to register",
                })),

            UserError::InvalidId => HttpResponse::build()
                .status(StatusCode::BAD_REQUEST)
                .body(json!({
                    "field": "id",
                    "message": "The provided id is not valid",
                })),
        }
    }
}
