use utoipa::ToSchema;

use super::models::UserRole;

#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub full_name: String,
    pub role: UserRole,
}
