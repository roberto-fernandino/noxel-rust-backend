use utoipa::ToSchema;

use super::models::User;

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct ListUsersResponse {
    pub users: Vec<User>,
}
