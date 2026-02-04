use utoipa::ToSchema;

use super::models::Ticket;

#[derive(Debug, serde::Serialize, ToSchema)]
pub struct ListTicketsResponse {
    pub tickets: Vec<Ticket>,
}
