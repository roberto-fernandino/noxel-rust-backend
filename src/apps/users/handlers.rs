use axum::{
    extract::State,
    http::StatusCode,
    Extension, Json,
};
use tracing::info;

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
    // Never log raw passwords.
    info!(
        target: "api.users.signup",
        flow = "organizer",
        full_name = %req.full_name,
        email = ?req.email,
        gov_identification = ?req.gov_identification,
        birth_date = ?req.birth_date,
        password_len = req.password.len(),
        "signup request"
    );

    let (user, _org) = super::sql::create_organizer_with_data(&state.db, req).await?;

    info!(
        target: "api.users.signup",
        flow = "organizer",
        user_id = %user.id,
        role = "organizer",
        email = ?user.email,
        gov_identification = ?user.gov_identification,
        birth_date = ?user.birth_date,
        status = 201,
        "signup response"
    );

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
    // Never log raw passwords.
    info!(
        target: "api.users.signup",
        flow = "attendee",
        full_name = %req.full_name,
        email = ?req.email,
        gov_identification = ?req.gov_identification,
        birth_date = ?req.birth_date,
        password_len = req.password.len(),
        "signup request"
    );

    let (user, _consumer) = super::sql::create_attendee_with_data(&state.db, req).await?;

    info!(
        target: "api.users.signup",
        flow = "attendee",
        user_id = %user.id,
        role = "attendee",
        email = ?user.email,
        gov_identification = ?user.gov_identification,
        birth_date = ?user.birth_date,
        status = 201,
        "signup response"
    );

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
    info!(
        target: "api.users.me",
        user_id = %auth_context.user.id,
        role = %auth_context.user.role.as_str(),
        "get_me request"
    );

    let related_data = match auth_context.user.role {
        UserRole::Organizer => Some(RelatedData::Organizer(
            OrganizerData::get_data(&state.db, auth_context.user.id).await?,
        )),
        UserRole::Attendee => Some(RelatedData::Attendee(
            AttendeeData::get_data(&state.db, auth_context.user.id).await?,
        )),
        _ => None,
    };

    info!(
        target: "api.users.me",
        user_id = %auth_context.user.id,
        status = 200,
        has_related_data = related_data.is_some(),
        "get_me response"
    );

    Ok((
        StatusCode::OK,
        Json(UserWithRelatedData {
            user: auth_context.user.clone(),
            related_data,
        }),
    ))
}
