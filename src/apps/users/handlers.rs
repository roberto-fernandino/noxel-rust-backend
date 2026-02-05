use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{results::ApiResult, AppState};

use super::{
    models::User,
    requests::{SignupAttendeeRequest, SignupOrganizerRequest},
};

#[utoipa::path(
    post,
    path = "/users/signup/organizer",
    request_body = SignupOrganizerRequest,
    responses(
        (status = 201, description = "Organizer signup", body = User)
    )
)]
pub async fn signup_organizer(
    State(state): State<AppState>,
    Json(req): Json<SignupOrganizerRequest>,
) -> ApiResult<StatusCode, User> {
    let (user, _org) = super::sql::create_organizer_with_data(&state.db, req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    post,
    path = "/users/signup/attendee",
    request_body = SignupAttendeeRequest,
    responses(
        (status = 201, description = "Attendee signup", body = User)
    )
)]
pub async fn signup_attendee(
    State(state): State<AppState>,
    Json(req): Json<SignupAttendeeRequest>,
) -> ApiResult<StatusCode, User> {
    let (user, _consumer) = super::sql::create_attendee_with_data(&state.db, req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}
