use axum::routing::{delete, get, patch, post, Router};

use super::controllers::*;
use crate::shared::infrastructure::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/", post(create_user))
        .route("/users/{id}", patch(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(state)
}
