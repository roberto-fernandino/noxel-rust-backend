use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{middleware::auth::AuthContext, AppError, AppState};

use super::{models::User, requests::CreateUserRequest};

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
    // TODO: Implement SQLx query via users::sql
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
    // TODO: Insert into DB via users::sql
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
    // TODO: Fetch from DB via users::sql
    Err((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "not found"})),
    ))
}
