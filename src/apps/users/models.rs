use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Roles supported by the system.
///
/// Naming: use `Attendee` for the ticket-buying/consuming role.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Organizer,
    Attendee,
    Admin,
}

#[derive(Debug, Clone, Serialize, ToSchema, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub role: UserRole,
}
