use utoipa::ToSchema;

use crate::apps::users::models::RelatedData;

use super::models::User;

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct ListUsersResponse {
    pub users: Vec<User>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct UserWithRelatedData {
    pub user: User,
    pub related_data: Option<RelatedData>,
}
