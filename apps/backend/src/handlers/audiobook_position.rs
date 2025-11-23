use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionResponse {
    pub id: Uuid,
    pub position_iso: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPosition {
    pub position_iso: String,
}

fn seconds_to_iso(seconds: u64) -> String {
    format!("PT{}S", seconds)
}

/// Get the current playback position for an audiobook.
/// Returns ISO8601 duration string.
pub async fn get_audiobook_position(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    // Placeholder
    let response = PositionResponse {
        id,
        position_iso: seconds_to_iso(0),
    };
    (StatusCode::OK, Json(response))
}

/// Set the playback position for an audiobook.
/// Accepts ISO8601 duration string.
pub async fn set_audiobook_position(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(payload): Json<SetPosition>,
) -> impl IntoResponse {
    // Placeholder
    let _ = payload;
    StatusCode::OK
}
