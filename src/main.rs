mod apps;
mod error;
mod middleware;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware as axum_middleware,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

type AppError = ApiError;

#[derive(Debug, Serialize, ToSchema)]
struct HealthResponse {
    ok: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Todo {
    id: Uuid,
    title: String,
    done: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
struct CreateTodoRequest {
    title: String,
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { ok: true })
}

#[utoipa::path(
    post,
    path = "/todos",
    request_body = CreateTodoRequest,
    responses(
        (status = 201, description = "Todo created", body = Todo),
        (status = 500, description = "Server error")
    )
)]
async fn create_todo(
    State(state): State<AppState>,
    Json(req): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    // NOTE: This expects the migrations in ./migrations to have been applied.
    let rec = sqlx::query!(
        r#"INSERT INTO todos (title) VALUES ($1)
           RETURNING id, title, done"#,
        req.title
    )
    .fetch_one(&state.db)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(Todo {
            id: rec.id,
            title: rec.title,
            done: rec.done,
        }),
    ))
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    params(
        ("id" = String, Path, description = "Todo id (uuid)")
    ),
    responses(
        (status = 200, description = "Todo", body = Todo),
        (status = 404, description = "Not found"),
        (status = 500, description = "Server error")
    )
)]
async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Todo>, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid uuid"})),
        )
    })?;

    let rec = sqlx::query!("SELECT id, title, done FROM todos WHERE id = $1", id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "database error"})),
            )
        })?;

    match rec {
        Some(rec) => Ok(Json(Todo {
            id: rec.id,
            title: rec.title,
            done: rec.done,
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "not found"})),
        )),
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        health,
        create_todo,
        get_todo,
        apps::users::handlers::list_users,
        apps::users::handlers::create_user,
        apps::users::handlers::get_user,
        apps::users::handlers::signup_organizer,
        apps::users::handlers::signup_attendee,
        apps::tickets::handlers::list_tickets,
        apps::tickets::handlers::create_ticket,
        apps::tickets::handlers::get_ticket
    ),
    components(schemas(
        HealthResponse,
        Todo,
        CreateTodoRequest,
        apps::users::models::User,
        apps::users::requests::CreateUserRequest,
        apps::users::requests::SignupRequest,
        apps::users::models::UserRole,
        apps::users::models::OrganizerData,
        apps::users::models::ConsumerData,
        apps::tickets::models::Ticket,
        apps::tickets::requests::CreateTicketRequest
    )),
    tags(
        (name = "noxel", description = "Noxel Rust Backend")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            // Default to info, make SQLx logs visible when needed.
            "info,sqlx=warn".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").map_err(|_| AppError::MissingDatabaseUrl)?;

    let db: Pool<Postgres> = PgPool::connect(&database_url).await?;

    let state = AppState { db };

    // Users: keep signup public; protect everything else.
    let users = Router::new()
        .merge(apps::users::public_router())
        .merge(
            apps::users::protected_router()
                .layer(axum_middleware::from_fn(middleware::auth::require_auth)),
        );

    let tickets = apps::tickets::router().layer(axum_middleware::from_fn(middleware::auth::require_auth));

    let app = Router::new()
        .route("/health", get(health))
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo))
        .nest("/users", users)
        .nest("/tickets", tickets)
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(state);

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
