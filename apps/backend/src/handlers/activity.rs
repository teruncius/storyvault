use crate::auth::AuthenticatedUser;
use crate::projections::audiobook_user_history::AudiobookUserHistoryProjection;
use crate::{AppState, projections::audiobook_user_history::ProjectionData};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ActivityResponse {
    audiobook: AudiobookResponse,
    accessed_at: DateTime<Utc>,
    r#type: String,
}

impl ActivityResponse {
    fn from(history: &ProjectionData) -> Self {
        Self {
            accessed_at: history.accessed_at,
            r#type: "audiobook".to_string(),
            audiobook: AudiobookResponse {
                id: history.audiobook_id,
            },
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AudiobookResponse {
    id: Uuid,
}

pub async fn get_recent_activity(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> impl IntoResponse {
    let projection = AudiobookUserHistoryProjection::new(state.db_pool);
    let history = projection
        .get_recent_activity(user.id)
        .await
        .unwrap_or_default();

    let response = history
        .into_iter()
        .map(|history| ActivityResponse::from(&history))
        .collect::<Vec<ActivityResponse>>();

    (StatusCode::OK, Json(response)).into_response()
}
