use sqlx::PgPool;
use uuid::Uuid;

use super::models::Ticket;

// NOTE: placeholder stubs. Implement with SQLx queries.

pub async fn list_tickets(_db: &PgPool) -> Result<Vec<Ticket>, sqlx::Error> {
    Ok(vec![])
}

pub async fn get_ticket_by_id(_db: &PgPool, _id: Uuid) -> Result<Option<Ticket>, sqlx::Error> {
    Ok(None)
}

pub async fn create_ticket(_db: &PgPool, _title: String) -> Result<Ticket, sqlx::Error> {
    Ok(Ticket {
        id: Uuid::new_v4(),
        title: "TODO".to_string(),
    })
}
