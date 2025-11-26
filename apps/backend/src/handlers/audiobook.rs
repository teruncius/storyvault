use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;
use crate::Audiobook;
use crate::auth::AuthenticatedUser;
use crate::iso8601::seconds_to_duration;
use crate::projections::audiobook_user_progress::AudiobookUserProgressProjection;

#[derive(Serialize)]
pub struct AudiobookResponse {
    id: Uuid,
    title: String,
    author: String,
    year: u32,
    detail_url: String,
    cover_url: String,
    position_url: String,
    stream_url: String,
    position_iso: Option<String>,
    duration_iso: Option<String>,
}

impl AudiobookResponse {
    fn from_audiobook(book: &Audiobook, base_url: &str, position_seconds: Option<u64>) -> Self {
        AudiobookResponse {
            id: book.id,
            title: book.title.clone(),
            author: book.author.clone(),
            year: book.year,
            detail_url: format!("{}/audiobook/{}", base_url, book.id),
            cover_url: format!("{}/audiobook/{}/cover", base_url, book.id),
            position_url: format!("{}/audiobook/{}/position", base_url, book.id),
            stream_url: format!("{}/audiobook/{}/stream", base_url, book.id),
            position_iso: position_seconds.map(seconds_to_duration),
            duration_iso: book.duration_seconds.map(seconds_to_duration),
        }
    }
}

pub async fn list_audiobooks(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> impl IntoResponse {
    let base_url = state.config.url.base.clone();

    // Load all positions for this user
    let projection = AudiobookUserProgressProjection::new(state.db_pool.clone());
    let positions = projection
        .get_all_positions(user.id)
        .await
        .unwrap_or_default();

    // Clone the books we need, then drop the lock
    let books: Vec<Audiobook> = { state.audiobooks.read().unwrap().values().cloned().collect() };

    let response: Vec<AudiobookResponse> = books
        .iter()
        .map(|b| {
            let position = positions.get(&b.id).copied();
            AudiobookResponse::from_audiobook(b, &base_url, position)
        })
        .collect();
    (StatusCode::OK, Json(response)).into_response()
}

pub async fn get_audiobook(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let base_url = state.config.url.base.clone();

    // Clone the audiobook, then drop the lock
    let book = {
        let books_guard = state.audiobooks.read().unwrap();
        books_guard.get(&id).cloned()
    };

    let book = match book {
        Some(b) => b,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    // Load position for this audiobook and user
    let projection = AudiobookUserProgressProjection::new(state.db_pool.clone());
    let position = projection.get_position(id, user.id).await.unwrap_or(None);

    let response = AudiobookResponse::from_audiobook(&book, &base_url, position);
    (StatusCode::OK, Json(response)).into_response()
}
