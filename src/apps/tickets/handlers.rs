use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{middleware::auth::AuthContext, AppError, AppState};

use super::{models::Ticket, requests::CreateTicketRequest};

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
    // TODO: Implement SQLx query via tickets::sql
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
    // TODO: Insert into DB via tickets::sql
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
    // TODO: Fetch from DB via tickets::sql
    Err((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "not found"})),
    ))
}
