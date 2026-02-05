use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    apps::users::{
        dto::UserWithRelatedData,
        models::{AttendeeData, OrganizerData, RelatedData, UserRole},
    },
    middleware::auth::AuthContext,
    results::ApiResult,
    AppState,
};

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

#[utoipa::path(
    get,
    path = "/users/me",
    responses(
        (status = 200, description = "Get current user", body = User)
    )
)]
pub async fn get_me(
    Extension(auth_context): Extension<AuthContext>,
    State(state): State<AppState>,
) -> ApiResult<StatusCode, UserWithRelatedData> {
    Ok((
        StatusCode::OK,
        Json(UserWithRelatedData {
            user: auth_context.user.clone(),
            related_data: match auth_context.user.role {
                UserRole::Organizer => Some(RelatedData::Organizer(
                    OrganizerData::get_data(&state.db, auth_context.user.id).await?,
                )),
                UserRole::Attendee => Some(RelatedData::Attendee(
                    AttendeeData::get_data(&state.db, auth_context.user.id).await?,
                )),
                _ => None,
            },
        }),
    ))
}
