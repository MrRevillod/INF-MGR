//! Error handling system for the application
//!
//! This module provides a structured error handling system that replaces
//! the generic `Input` struct with specific error types for better
//! type safety and developer experience.

mod app_error;
mod auth;
mod http;
mod not_found;
mod validation;

// Re-export main types
pub use app_error::AppError;
pub use auth::AuthError;
pub use not_found::NotFoundError;
pub use validation::ValidationError;

// Convenience type alias
pub type AppResult<T> = Result<T, AppError>;
