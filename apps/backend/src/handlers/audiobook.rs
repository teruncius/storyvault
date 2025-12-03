use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::Audiobook;
use crate::auth::AuthenticatedUser;
use crate::projections::audiobook_user_progress::AudiobookUserProgressProjection;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudiobookResponse {
    id: Uuid,
    title: String,
    authors: Vec<String>,
    year: u32,
    detail_url: String,
    cover_url: String,
    position_url: String,
    stream_url: String,
    position_seconds: Option<u64>,
    runtime_seconds: u64,
    sample_rate_hz: u32,
    bit_rate_kbps: u64,
}

impl AudiobookResponse {
    fn from_audiobook(book: &Audiobook, base_url: &str, position_seconds: Option<u64>) -> Self {
        AudiobookResponse {
            id: book.id,
            title: book.title.clone(),
            authors: book.authors.clone(),
            year: book.year,
            detail_url: format!("{}/api/audiobook/{}", base_url, book.id),
            cover_url: format!("{}/api/audiobook/{}/cover", base_url, book.id),
            position_url: format!("{}/api/audiobook/{}/position", base_url, book.id),
            stream_url: format!("{}/api/audiobook/{}/stream", base_url, book.id),
            position_seconds,
            runtime_seconds: book.duration_seconds,
            sample_rate_hz: book.sample_rate_hz,
            bit_rate_kbps: book.bit_rate_kbps,
        }
    }
}

#[derive(Deserialize)]
pub struct ListAudiobooksQuery {
    search: Option<String>,
}

pub async fn list_audiobooks(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(query): Query<ListAudiobooksQuery>,
) -> impl IntoResponse {
    let base_url = state.config.url.base.clone();

    // Load all positions for this user
    let projection = AudiobookUserProgressProjection::new(state.db_pool.clone());
    let positions = projection
        .get_all_positions(user.id)
        .await
        .unwrap_or_default();

    // Clone the books we need, then drop the lock
    let mut books: Vec<Audiobook> =
        { state.audiobooks.read().unwrap().values().cloned().collect() };

    // Filter by search term if provided
    if let Some(search_term) = query.search {
        let search_lower = search_term.to_lowercase();
        books.retain(|book| {
            book.title.to_lowercase().contains(&search_lower)
                || book
                    .authors
                    .iter()
                    .any(|author| author.to_lowercase().contains(&search_lower))
        });
    }

    // Sort by title
    books.sort_by(|a, b| a.title.cmp(&b.title));

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
