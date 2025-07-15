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

use serde_json::json;
use sword::web::HttpResponse;

use crate::features::user::domain::UserError;

// Each variant of the `UserError` enum corresponds to a specific error
// that can occur in the user management process.

impl From<UserError> for HttpResponse {
    fn from(value: UserError) -> Self {
        match value {
            UserError::EmailAlreadyExists => HttpResponse::Conflict().data(json!({
                "field": "email",
                "message": "Email already exists",
            })),

            UserError::NotFound => HttpResponse::NotFound().data(json!({
                "message": "User not found",
            })),

            UserError::UnexpectedError(error) => {
                eprintln!("| USER ERROR | - Unexpected error: {}", error);

                HttpResponse::InternalServerError().data(json!({
                    "message": "Unexpected error",
                }))
            }

            UserError::InvalidEmail => HttpResponse::BadRequest().data(json!({
                "field": "email",
                "message": "The provided email is not valid to register",
            })),

            UserError::IdAlreadyExists => HttpResponse::BadRequest().data(json!({
                "field": "id",
                "message": "User ID already exists",
            })),

            UserError::InvalidRole => HttpResponse::BadRequest().data(json!({
                "field": "roles",
                "message": "Invalid role provided",
            })),
        }
    }
}
