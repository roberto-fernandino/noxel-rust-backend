mod apps;
mod middleware;
mod results;
mod routes;
mod state;

use anyhow::Result;
use axum::{extract::Request, middleware::Next};
use sqlx::{PgPool, Pool, Postgres};
use std::net::SocketAddr;
use std::time::Instant;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{results::ApiError, state::AppState};

async fn log_requests(request: Request, next: Next) -> axum::response::Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();
    // Span so every log in this request (including error in IntoResponse) gets method + uri
    let span = tracing::info_span!("request", %method, %uri);
    let _guard = span.enter();
    tracing::info!("request");
    let response = next.run(request).await;
    let status = response.status();
    let elapsed_ms = start.elapsed().as_millis();
    tracing::info!(%status, elapsed_ms, "response");
    if status.is_server_error() {
        tracing::error!(%status, elapsed_ms, "response 5xx — see ERROR above for cause");
    }
    response
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    if dotenv::dotenv().is_ok() {
        eprintln!("Loaded .env file");
    } else {
        eprintln!("Warning: .env file not found, using system environment variables");
    }

    // Read RUST_LOG from .env file directly (prioritize .env over system env)
    let filter = std::fs::read_to_string(".env")
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find(|line| line.starts_with("RUST_LOG="))
                .and_then(|line| line.splitn(2, '=').nth(1).map(|s| s.trim().to_string()))
        })
        .or_else(|| std::env::var("RUST_LOG").ok())
        .unwrap_or_else(|| "info,sqlx=warn".to_string());

    // Print to stderr so we can see it immediately (before tracing is initialized)
    eprintln!("Initializing logging with RUST_LOG={}", filter);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_new(&filter).unwrap_or_else(|e| {
                eprintln!("Failed to parse RUST_LOG='{}': {}", filter, e);
                tracing_subscriber::EnvFilter::new("info,sqlx=warn")
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    tracing::debug!("Logging initialized with RUST_LOG={}", filter);
    tracing::info!("Application starting...");

    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => {
            tracing::debug!(
                "Connecting to database at {}",
                url.split('@').nth(1).unwrap_or("***")
            );
            url
        }
        Err(_) => {
            tracing::error!("DATABASE_URL environment variable not set");
            return Err(ApiError::MissingDatabaseUrl.into());
        }
    };

    tracing::info!("Connecting to database...");
    let db: Pool<Postgres> = PgPool::connect(&database_url).await?;
    tracing::info!("Database connection established");

    let state = AppState { db };

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let app = routes::router()
        .with_state(state)
        .layer(axum::middleware::from_fn(log_requests));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(%addr, "listening");

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();

    Ok(())
}
