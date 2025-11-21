use axum::{Json, extract::State};
use serde::Serialize;
use uuid::Uuid;

use crate::{AppState, Audiobook};

#[derive(Serialize)]
pub struct AudiobookResponse {
    id: Uuid,
    title: String,
    author: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<u32>,
    cover_url: String,
    position_url: String,
    stream_url: String,
}

impl AudiobookResponse {
    fn from_audiobook(book: &Audiobook, base_url: &str) -> Self {
        AudiobookResponse {
            id: book.id,
            title: book.title.clone(),
            author: book.author.clone(),
            year: book.year,
            cover_url: format!("{}/audiobook/{}/cover", base_url, book.id),
            position_url: format!("{}/audiobook/{}/position", base_url, book.id),
            stream_url: format!("{}/audiobook/{}/stream", base_url, book.id),
        }
    }
}

pub async fn list_audiobooks(State(state): State<AppState>) -> Json<Vec<AudiobookResponse>> {
    let books = state.audiobooks.read().unwrap();
    let base_url = &state.config.url.base;
    let response: Vec<AudiobookResponse> = books
        .values()
        .map(|b| AudiobookResponse::from_audiobook(b, base_url))
        .collect();
    Json(response)
}
