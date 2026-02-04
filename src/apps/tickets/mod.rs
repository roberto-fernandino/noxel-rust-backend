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
        .route("/", get(list_tickets))
        .route("/", post(create_ticket))
        .route("/:id", get(get_ticket))
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTicketRequest {
    pub title: String,
}

#[utoipa::path(
    get,
    path = "/tickets",
    responses(
        (status = 200, description = "List tickets", body = [Ticket])
    )
)]
pub async fn list_tickets(
    State(_state): State<AppState>,
    axum::extract::Extension(_auth): axum::extract::Extension<AuthContext>,
) -> Result<Json<Vec<Ticket>>, AppError> {
    // TODO: Implement SQLx query
    Ok(Json(vec![]))
}

#[utoipa::path(
    post,
    path = "/tickets",
    request_body = CreateTicketRequest,
    responses(
        (status = 201, description = "Ticket created", body = Ticket)
    )
)]
pub async fn create_ticket(
    State(_state): State<AppState>,
    axum::extract::Extension(_auth): axum::extract::Extension<AuthContext>,
    Json(req): Json<CreateTicketRequest>,
) -> Result<(StatusCode, Json<Ticket>), AppError> {
    // TODO: Insert into DB
    let ticket = Ticket {
        id: Uuid::new_v4(),
        title: req.title,
    };
    Ok((StatusCode::CREATED, Json(ticket)))
}

#[utoipa::path(
    get,
    path = "/tickets/{id}",
    params(
        ("id" = String, Path, description = "Ticket id (uuid)")
    ),
    responses(
        (status = 200, description = "Ticket", body = Ticket),
        (status = 404, description = "Not found")
    )
)]
pub async fn get_ticket(
    State(_state): State<AppState>,
    axum::extract::Extension(_auth): axum::extract::Extension<AuthContext>,
    Path(_id): Path<String>,
) -> Result<Json<Ticket>, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Fetch from DB
    Err((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "not found"})),
    ))
}
