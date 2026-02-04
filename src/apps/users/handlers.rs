use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{middleware::auth::AuthContext, AppState};

use super::{models::User, requests::CreateUserRequest};
