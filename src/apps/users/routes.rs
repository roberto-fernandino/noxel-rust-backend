use axum::{routing::{get, post}, Router};

use crate::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_users))
        .route("/", post(handlers::create_user))
        .route("/:id", get(handlers::get_user))
}
