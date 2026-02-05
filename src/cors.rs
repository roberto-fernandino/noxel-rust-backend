use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tower_http::cors::{Any, CorsLayer};

/// Parse CORS_ALLOWED_ORIGINS env var (comma-separated list) into allowed origins.
/// Example: CORS_ALLOWED_ORIGINS=http://localhost:3000,https://app.example.com
pub fn cors_layer_from_env() -> CorsLayer {
    let origins: Vec<axum::http::HeaderValue> = std::env::var("CORS_ALLOWED_ORIGINS")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            s.split(',')
                .map(|o| o.trim())
                .filter(|o| !o.is_empty())
                .filter_map(|o| o.parse().ok())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let layer =
        CorsLayer::new()
            .allow_methods(Any)
            .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE]);

    if origins.is_empty() {
        tracing::debug!("CORS: no CORS_ALLOWED_ORIGINS set, CORS layer allows no origins");
        layer
    } else {
        tracing::info!(count = origins.len(), origins = ?origins, "CORS allowed origins from CORS_ALLOWED_ORIGINS");
        layer.allow_origin(origins)
    }
}
