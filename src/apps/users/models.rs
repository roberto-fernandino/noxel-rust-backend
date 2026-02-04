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

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub role: UserRole,

    #[schema(nullable = true)]
    pub email: Option<String>,

    #[schema(nullable = true, example = 12345678901)]
    pub gov_identification: Option<i64>,

    #[schema(nullable = true, example = "1990-01-31")]
    pub birth_date: Option<chrono::NaiveDate>,
}
