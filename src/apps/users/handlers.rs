use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{middleware::auth::AuthContext, results::ApiResult, AppState};

use super::{models::User, requests::SignupRequest};

#[utoipa::path(
    post,
    path = "/users/signup/organizer",
    request_body = SignupRequest,
    responses(
        (status = 201, description = "Organizer signup", body = User)
    )
)]
pub async fn signup_organizer(
    State(state): State<AppState>,
    Json(req): Json<SignupRequest>,
) -> ApiResult<StatusCode, User> {
    let (user, _org) = super::sql::create_organizer_with_data(&state.db, req).await?;
    Ok((StatusCode::CREATED, user))
}

#[utoipa::path(
    post,
    path = "/users/signup/attendee",
    request_body = SignupRequest,
    responses(
        (status = 201, description = "Attendee signup", body = User)
    )
)]
pub async fn signup_attendee(
    State(state): State<AppState>,
    Json(req): Json<SignupRequest>,
) -> ApiResult<StatusCode, User> {
    let (user, _consumer) = super::sql::create_attendee_with_data(&state.db, req).await?;
    Ok((StatusCode::CREATED, user))
}
