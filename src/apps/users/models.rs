use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
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

    #[schema(nullable = false)]
    pub email: String,

    #[schema(nullable = false, example = 12345678901_i64)]
    pub gov_identification: i64,
}

/// Row returned from database for User (with role as string)
#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub full_name: String,
    pub role: String,
    pub email: String,
    pub gov_identification: i64,
}

impl UserRow {
    pub fn into_user(self) -> User {
        User {
            id: self.id,
            full_name: self.full_name,
            role: UserRole::from_str(&self.role).unwrap_or(UserRole::Attendee),
            email: self.email,
            gov_identification: self.gov_identification,
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
pub struct AttendeeData {
    pub id: Uuid,
    pub user_id: Uuid,
    #[schema(nullable = false, example = "+5511999999999")]
    pub phone: String,
    #[schema(nullable = false, example = "1990-01-31")]
    pub birth_date: chrono::NaiveDate,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum RelatedData {
    Organizer(OrganizerData),
    Attendee(AttendeeData),
}
