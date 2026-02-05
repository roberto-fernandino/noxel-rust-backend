use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::{apps::users::models::User, middleware};

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user: User,
}

/// Very small auth middleware placeholder.
/// Current behavior (intentionally simple):
/// - Requires `Authorization: Bearer <something>`
///
/// Replace this with real JWT/session validation.
pub async fn require_auth(mut req: Request, next: Next) -> impl IntoResponse {
    let auth = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let token = auth.strip_prefix("Bearer ").unwrap_or(auth);
    if token.trim().is_empty() {
        return (StatusCode::UNAUTHORIZED, "missing token").into_response();
    }

    let secret = match std::env::var("JWT_SECRET") {
        Ok(v) if !v.is_empty() => v,
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, "missing JWT_SECRET").into_response(),
    };

    let claims = match middleware::jwt::verify_token(token, &secret) {
        Ok(claims) => claims,
        Err(_) => return (StatusCode::UNAUTHORIZED, "invalid token").into_response(),
    };

    req.extensions_mut().insert(AuthContext {
        user: claims.user.clone(),
    });
    next.run(req).await
}
