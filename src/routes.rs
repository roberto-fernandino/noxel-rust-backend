use crate::{results::ApiResult, state::AppState};
use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Serialize, ToSchema)]
struct HealthResponse {
    ok: bool,
}
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
async fn health() -> ApiResult<StatusCode, HealthResponse> {
    Ok((StatusCode::OK, Json(HealthResponse { ok: true })))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        health,
        crate::apps::users::handlers::signup_organizer,
        crate::apps::users::handlers::signup_attendee,
    ),
    components(schemas(
        HealthResponse,
        crate::apps::users::dto::SignupResponse,
        crate::apps::users::models::User,
        crate::apps::users::requests::SignupAttendeeRequest,
        crate::apps::users::requests::SignupOrganizerRequest,
    )),
    tags(
        (name = "noxel", description = "Noxel Rust Backend")
    )
)]
struct ApiDoc;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .merge(crate::apps::users::routes::router())
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-doc/openapi.json", ApiDoc::openapi()),
        )
}
