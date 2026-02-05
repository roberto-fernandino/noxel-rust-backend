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
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let claims = match middleware::jwt::verify_token(token, &std::env::var("JWT_SECRET").unwrap()) {
        Ok(claims) => claims,
        Err(_) => return (StatusCode::UNAUTHORIZED, "invalid token").into_response(),
    };
    req.extensions_mut().insert(AuthContext {
        user: claims.user.clone(),
    });
    next.run(req).await
}
