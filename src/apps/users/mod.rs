use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{middleware::auth::AuthContext, AppError, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
        .route("/", post(create_user))
        .route("/:id", get(get_user))
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Organizer,
    Attendee,
    Admin,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub role: UserRole,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub full_name: String,
    pub role: UserRole,
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List users", body = [User])
    )
)]
pub async fn list_users(
    State(_state): State<AppState>,
    axum::extract::Extension(_auth): axum::extract::Extension<AuthContext>,
) -> Result<Json<Vec<User>>, AppError> {
    // TODO: Implement SQLx query
    Ok(Json(vec![]))
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = User)
    )
)]
pub async fn create_user(
    State(_state): State<AppState>,
    axum::extract::Extension(_auth): axum::extract::Extension<AuthContext>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    // TODO: Insert into DB
    let user = User {
        id: Uuid::new_v4(),
        full_name: req.full_name,
        role: req.role,
    };
    Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User id (uuid)")
    ),
    responses(
        (status = 200, description = "User", body = User),
        (status = 404, description = "Not found")
    )
)]
pub async fn get_user(
    State(_state): State<AppState>,
    axum::extract::Extension(_auth): axum::extract::Extension<AuthContext>,
    Path(_id): Path<String>,
) -> Result<Json<User>, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Fetch from DB
    Err((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "not found"})),
    ))
}
