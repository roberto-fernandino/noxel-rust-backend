use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, ToSchema)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
}
