use utoipa::ToSchema;

/// Request body for public signup endpoints.
/// Role is inferred from the endpoint (organizer or attendee).
#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct SignupOrganizerRequest {
    pub full_name: String,

    /// User's password (will be hashed with Argon2id)
    pub password: String,

    #[schema(nullable = true)]
    pub email: Option<String>,

    /// CPF (11 digits) or CNPJ (14 digits)
    #[schema(nullable = true, example = 12345678901_i64)]
    pub gov_identification: Option<i64>,

    #[schema(nullable = true, example = "1990-01-31")]
    pub birth_date: Option<chrono::NaiveDate>,
}
/// Request body for public signup endpoints.
/// Role is inferred from the endpoint (organizer or attendee).
#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct SignupAttendeeRequest {
    #[schema(nullable = false)]
    pub full_name: String,

    /// User's password (will be hashed with Argon2id)
    pub password: String,

    #[schema(nullable = false, example = "+5511999999999")]
    pub phone: Option<String>,

    #[schema(nullable = false)]
    pub email: Option<String>,

    /// CPF (11 digits) or CNPJ (14 digits)
    #[schema(nullable = false, example = 12345678901_i64)]
    pub gov_identification: Option<i64>,

    #[schema(nullable = false, example = "1990-01-31")]
    pub birth_date: chrono::NaiveDate,
}
