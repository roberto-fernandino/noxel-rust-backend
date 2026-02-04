use axum::{routing::{get, post}, Router};

use crate::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_tickets))
        .route("/", post(handlers::create_ticket))
        .route("/:id", get(handlers::get_ticket))
}
