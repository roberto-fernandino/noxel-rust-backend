use utoipa::ToSchema;

#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct UserAddressRequest {
    /// CEP (Brazilian postal code)
    #[schema(nullable = false, example = "01001-000")]
    pub cep: String,

    #[schema(nullable = false, example = "Avenida Paulista")]
    pub logradouro: String,

    #[schema(nullable = false, example = "123")]
    pub numero: String,

    #[schema(nullable = true, example = "Apto 12")]
    pub complemento: Option<String>,

    #[schema(nullable = true, example = "Centro")]
    pub bairro: Option<String>,

    #[schema(nullable = false, example = "São Paulo")]
    pub cidade: String,

    /// State abbreviation (e.g. SP)
    #[schema(nullable = false, example = "SP")]
    pub estado: String,
}

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

    #[schema(nullable = false)]
    pub address: UserAddressRequest,
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

    #[schema(nullable = false)]
    pub address: UserAddressRequest,
}
