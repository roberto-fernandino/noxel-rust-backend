use axum::{routing::{get, post}, Router};

use crate::AppState;

use super::handlers;

/// Unauthenticated endpoints.
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/signup/organizer", post(handlers::signup_organizer))
        .route("/signup/attendee", post(handlers::signup_attendee))
}

/// Authenticated endpoints.
pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_users))
        .route("/", post(handlers::create_user))
        .route("/:id", get(handlers::get_user))
}

/// Convenience router (no auth layers applied here).
pub fn router() -> Router<AppState> {
    Router::new().merge(public_router()).merge(protected_router())
}
