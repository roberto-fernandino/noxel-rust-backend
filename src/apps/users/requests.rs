use utoipa::ToSchema;

use super::models::UserRole;

#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub full_name: String,
    pub role: UserRole,

    pub email: Option<String>,

    /// CPF (11 digits) or CNPJ (14 digits)
    pub gov_identification: Option<i64>,

    pub birth_date: Option<chrono::NaiveDate>,
}

/// Request body for public signup endpoints.
/// Role is inferred from the endpoint (organizer or attendee).
#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct SignupRequest {
    pub full_name: String,

    /// User's password (will be hashed with Argon2id)
    pub password: String,

    #[schema(nullable = true)]
    pub email: Option<String>,

    /// CPF (11 digits) or CNPJ (14 digits)
    #[schema(nullable = true, example = 12345678901)]
    pub gov_identification: Option<i64>,

    #[schema(nullable = true, example = "1990-01-31")]
    pub birth_date: Option<chrono::NaiveDate>,
}
