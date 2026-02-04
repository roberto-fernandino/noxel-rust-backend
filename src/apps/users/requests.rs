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
