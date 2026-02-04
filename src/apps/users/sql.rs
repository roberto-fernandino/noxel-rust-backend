use sqlx::PgPool;
use uuid::Uuid;

use super::{models::User, models::UserRole};

// NOTE: placeholder stubs. Implement with SQLx queries.

pub async fn list_users(_db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    Ok(vec![])
}

pub async fn get_user_by_id(_db: &PgPool, _id: Uuid) -> Result<Option<User>, sqlx::Error> {
    Ok(None)
}

pub async fn create_user(_db: &PgPool, _full_name: String, _role: UserRole) -> Result<User, sqlx::Error> {
    Ok(User {
        id: Uuid::new_v4(),
        full_name: "TODO".to_string(),
        role: UserRole::Attendee,
    })
}
