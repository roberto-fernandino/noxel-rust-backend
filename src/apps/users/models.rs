use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::results::ApiError;

/// Roles supported by the system.
///
/// Naming: use `Attendee` for the ticket-buying/consuming role.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Organizer,   // Producer
    Attendee,    // Consumer
    Admin,       // Us
    Promoter,    // Promoter
    Colaborator, // Colaborator
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Organizer => "organizer",
            UserRole::Attendee => "attendee",
            UserRole::Admin => "admin",
            UserRole::Promoter => "promoter",
            UserRole::Colaborator => "colaborator",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, ApiError> {
        match s.to_lowercase().as_str() {
            "organizer" => Ok(UserRole::Organizer),
            "attendee" => Ok(UserRole::Attendee),
            "admin" => Ok(UserRole::Admin),
            "promoter" => Ok(UserRole::Promoter),
            "colaborator" => Ok(UserRole::Colaborator),
            _ => Err(ApiError::InvalidRole(s.to_string())),
        }
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Attendee
    }
}

#[derive(Debug, Clone, Serialize, ToSchema, FromRow, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    #[sqlx(skip)]
    pub role: UserRole,

    #[schema(nullable = true)]
    pub email: Option<String>,

    #[schema(nullable = true, example = 12345678901_i64)]
    pub gov_identification: Option<i64>,

    #[schema(nullable = true, example = "1990-01-31")]
    pub birth_date: Option<chrono::NaiveDate>,
}

/// Row returned from database for User (with role as string)
#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub full_name: String,
    pub role: String,
    pub email: Option<String>,
    pub gov_identification: Option<i64>,
    pub birth_date: Option<chrono::NaiveDate>,
}

impl UserRow {
    pub fn into_user(self) -> User {
        User {
            id: self.id,
            full_name: self.full_name,
            role: UserRole::from_str(&self.role).unwrap_or(UserRole::Attendee),
            email: self.email,
            gov_identification: self.gov_identification,
            birth_date: self.birth_date,
        }
    }
}

/// Organizer-specific data (1:1 with users where role = organizer)
#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct OrganizerData {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Consumer/Attendee-specific data (1:1 with users where role = attendee)
#[derive(Debug, Clone, Serialize, ToSchema, FromRow)]
pub struct ConsumerData {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
