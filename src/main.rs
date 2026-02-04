mod apps;
mod error;
mod middleware;

use anyhow::Result;
use axum::Json;
use serde::Serialize;
use sqlx::{PgPool, Pool, Postgres};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{OpenApi, ToSchema};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

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
async fn health() -> Result<Json<HealthResponse>> {
    Ok(Json(HealthResponse { ok: true }))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        health,
    ),
    components(schemas(
        HealthResponse,
        apps::users::models::User,
    )),
    tags(
        (name = "noxel", description = "Noxel Rust Backend")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // Default to info, make SQLx logs visible when needed.
                "info,sqlx=warn".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return Err(anyhow::anyhow!("DATABASE_URL is not set")),
    };
    let db: Pool<Postgres> = PgPool::connect(&database_url).await?;

    let state = AppState { db };

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(%addr, "listening");

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();

    Ok(())
}
