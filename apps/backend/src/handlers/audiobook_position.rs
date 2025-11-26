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
    events::{AudiobookProgressPayload, Event, EventPayload, EventStore, ProgressType},
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

fn seconds_to_iso(seconds: i64) -> String {
    format!("PT{}S", seconds)
}

fn iso_to_seconds(iso: &str) -> Option<i64> {
    // Simple parser for PT#S format
    if iso.starts_with("PT") && iso.ends_with('S') {
        let seconds_str = &iso[2..iso.len() - 1];
        seconds_str.parse::<i64>().ok()
    } else {
        None
    }
}

/// Get the current playback position for an audiobook.
/// Returns ISO8601 duration string.
pub async fn get_audiobook_position(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if !state.audiobooks.read().unwrap().contains_key(&id) {
        return StatusCode::NOT_FOUND.into_response();
    }

    let event_store = EventStore::new(state.db_pool.clone());
    let position_seconds = event_store
        .get_latest_position(id, user.id)
        .await
        .unwrap_or(None)
        .unwrap_or(0);

    let response = PositionResponse {
        position_iso: seconds_to_iso(position_seconds),
    };

    (StatusCode::OK, Json(response)).into_response()
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

    let position_seconds = match iso_to_seconds(&payload.position_iso) {
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
