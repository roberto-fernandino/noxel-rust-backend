use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
}
