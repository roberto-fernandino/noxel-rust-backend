use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use utoipa::ToSchema;

/// Consistent error payload for the API.
#[derive(Debug, Clone, serde::Serialize, ToSchema)]
pub struct ApiErrorBody {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("invalid role: {0}")]
    InvalidRole(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("not found")]
    NotFound,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("missing env var DATABASE_URL")]
    MissingDatabaseUrl,

    #[error("database error")]
    Db(#[from] sqlx::Error),

    #[error("internal server error")]
    Internal,
}

impl ApiError {
    pub fn status(&self) -> StatusCode {
        match self {
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::MissingDatabaseUrl => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InvalidRole(_) => StatusCode::BAD_REQUEST,
        }
    }

    pub fn code(&self) -> Option<&'static str> {
        match self {
            ApiError::Unauthorized => Some("unauthorized"),
            ApiError::Forbidden => Some("forbidden"),
            ApiError::NotFound => Some("not_found"),
            ApiError::BadRequest(_) => Some("bad_request"),
            ApiError::MissingDatabaseUrl => Some("missing_database_url"),
            ApiError::Db(_) => Some("db_error"),
            ApiError::Internal => Some("internal"),
            ApiError::InvalidRole(_) => Some("invalid_role"),
        }
    }
}

/// Build full cause chain so logs show the actual root cause (e.g. Postgres message).
fn error_chain(err: &(dyn std::error::Error + 'static)) -> String {
    let mut s = err.to_string();
    let mut src = err.source();
    while let Some(e) = src {
        s.push_str(" → ");
        s.push_str(&e.to_string());
        src = e.source();
    }
    s
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status();
        if status.is_server_error() {
            match &self {
                ApiError::Db(e) => {
                    let chain = error_chain(e);
                    tracing::error!(
                        api_error = %self,
                        code = ?self.code(),
                        cause_chain = %chain,
                        "API ERROR 5xx — read cause_chain for root cause (e.g. missing table/column)"
                    );
                }
                _ => {
                    tracing::error!(
                        api_error = %self,
                        code = ?self.code(),
                        "API ERROR 5xx"
                    );
                }
            }
        } else {
            tracing::debug!(api_error = %self, status = %status, "API error 4xx");
        }
        let body = ApiErrorBody {
            error: self.to_string(),
            code: self.code().map(|s| s.to_string()),
        };
        (status, Json(body)).into_response()
    }
}

pub type ApiResult<S, T> = Result<(S, Json<T>), ApiError>;
