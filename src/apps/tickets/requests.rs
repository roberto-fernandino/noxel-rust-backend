use utoipa::ToSchema;

#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct CreateTicketRequest {
    pub title: String,
}
