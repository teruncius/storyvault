use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    auth::AuthenticatedUser,
    events::{AudiobookProgressPayload, Event, EventPayload, ProgressType},
    iso8601::duration_to_seconds,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionResponse {
    pub position_iso: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPosition {
    pub event_type: String,
    pub position_iso: String,
}

/// Set the playback position for an audiobook.
/// Accepts ISO8601 duration string.
pub async fn set_audiobook_position(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<SetPosition>,
) -> impl IntoResponse {
    let event_type = match payload.event_type.parse::<ProgressType>() {
        Ok(et) => et,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid event type").into_response(),
    };

    let position_seconds = match duration_to_seconds(&payload.position_iso) {
        Some(s) => s,
        None => return (StatusCode::BAD_REQUEST, "Invalid position format").into_response(),
    };

    // Create the event with new structure
    let event = Event {
        event_id: Uuid::new_v4(),
        created_at: chrono::Utc::now(),
        payload: EventPayload::AudiobookProgress(AudiobookProgressPayload {
            audiobook_id: id,
            user_id: user.id,
            event_type,
            position_seconds,
        }),
    };

    // Enqueue the event for background processing
    match state.event_queue.enqueue(event) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            eprintln!("Failed to enqueue event: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to enqueue event").into_response()
        }
    }
}
