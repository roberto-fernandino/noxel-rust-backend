use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: String,
    pub role: Role,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Organizer,
    Attendee,
    Admin,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Organizer => "organizer",
            Role::Attendee => "attendee",
            Role::Admin => "admin",
        }
    }
}

/// Very small auth middleware placeholder.
///
/// Current behavior (intentionally simple):
/// - Requires `Authorization: Bearer <something>`
/// - Optionally reads `x-user-id` and `x-user-role`.
///
/// Replace this with real JWT/session validation.
pub async fn require_auth(mut req: Request, next: Next) -> impl IntoResponse {
    let auth = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !auth.starts_with("Bearer ") || auth.trim() == "Bearer" {
        return (StatusCode::UNAUTHORIZED, "missing/invalid bearer token").into_response();
    }

    let user_id = req
        .headers()
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("anonymous")
        .to_string();

    let role = match req
        .headers()
        .get("x-user-role")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("attendee")
        .to_ascii_lowercase()
        .as_str()
    {
        "organizer" => Role::Organizer,
        "admin" => Role::Admin,
        _ => Role::Attendee,
    };

    req.extensions_mut().insert(AuthContext { user_id, role });
    next.run(req).await
}
