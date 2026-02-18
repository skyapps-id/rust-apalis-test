use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::server::rest::ServerState;

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub event_id: String,
    pub device_uuid: String,
}

pub async fn create_order(
    State(state): State<ServerState>,
    Json(payload): Json<CreateOrderRequest>,
) -> impl IntoResponse {
    match state.container.order_service.create_order(payload.event_id.clone(), payload.device_uuid).await {
        Ok(event_id) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "message": "Order task scheduled successfully",
                "event_id": event_id
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to schedule task: {}", e)
            })),
        )
            .into_response(),
    }
}
