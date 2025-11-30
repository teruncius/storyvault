use crate::AppState;
use crate::auth::AuthenticatedUser;
use crate::projections::audiobook_user_history::AudiobookUserHistoryProjection;
use crate::state::Audiobook;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

pub async fn get_recent_audiobooks(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> impl IntoResponse {
    let projection = AudiobookUserHistoryProjection::new(state.db_pool);
    let history = projection
        .get_recent_audiobooks(user.id)
        .await
        .unwrap_or_default();

    let books: HashMap<Uuid, Audiobook> = state.audiobooks.read().unwrap().clone();

    info!("Recent history: {:#?}", history);
    info!("Recent audiobooks: {:#?}", books);

    (StatusCode::OK, Json(history)).into_response()
}
