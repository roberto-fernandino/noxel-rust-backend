use utoipa::ToSchema;

/// Request body for public signup endpoints.
/// Role is inferred from the endpoint (organizer or attendee).
#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct SignupOrganizerRequest {
    #[schema(nullable = false, example = "Johnson Smith")]
    pub full_name: String,

    /// User's password (will be hashed with Argon2id)
    #[schema(nullable = false, example = "123456")]
    pub password: String,

    #[schema(nullable = false, example = "johnson@noxel.com")]
    pub email: String,

    /// CPF (11 digits) or CNPJ (14 digits)
    #[schema(nullable = false, example = 12345678901_i64)]
    pub gov_identification: i64,
}
/// Request body for public signup endpoints.
/// Role is inferred from the endpoint (organizer or attendee).
#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct SignupAttendeeRequest {
    #[schema(nullable = false, example = "Robert Johnson Smith Junior the Third")]
    pub full_name: String,

    /// User's password (will be hashed with Argon2id)
    #[schema(nullable = false, example = "123456")]
    pub password: String,

    #[schema(nullable = false, example = "+5511999999999")]
    pub phone: String,

    #[schema(nullable = false, example = "robert@noxel.com")]
    pub email: String,

    /// CPF (11 digits) or CNPJ (14 digits)
    #[schema(nullable = false, example = 12345678901_i64)]
    pub gov_identification: i64,

    #[schema(nullable = false, example = "1990-01-31")]
    pub birth_date: chrono::NaiveDate,
}
