use axum::routing::{delete, get, patch, post, Router};

use super::controllers::*;
use crate::shared::infrastructure::DependencyContainer;

pub fn router(state: DependencyContainer) -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/", post(create_user))
        .route("/users/{id}", patch(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(state)
}
