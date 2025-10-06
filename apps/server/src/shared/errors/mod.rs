mod app_error;
mod auth;
mod http;
mod not_found;
mod validation;

pub use app_error::AppError;
pub use auth::AuthError;
pub use not_found::NotFoundError;
pub use validation::ValidationError;

pub type AppResult<T> = Result<T, AppError>;
