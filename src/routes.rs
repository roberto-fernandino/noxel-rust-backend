use crate::{results::ApiResult, state::AppState};
use axum::{routing::get, Json, Router};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

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
async fn health() -> ApiResult<Json<HealthResponse>> {
    Ok(Json(HealthResponse { ok: true }))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        health,
    ),
    components(schemas(
        HealthResponse,
    )),
    tags(
        (name = "noxel", description = "Noxel Rust Backend")
    )
)]
struct ApiDoc;
pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}
