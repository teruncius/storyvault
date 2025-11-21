use axum::{
    extract::{Path as AxumPath, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::AppState;

pub async fn get_audiobook_cover(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<Uuid>,
) -> Response {
    let book_path = {
        let books = state.audiobooks.read().unwrap();
        books.iter().find(|b| b.id == id).map(|b| b.path.clone())
    };

    if let Some(path) = book_path {
        if let Some(parent) = path.parent() {
            // Try WebP first
            let cover_webp = parent.join("cover.webp");
            if cover_webp.exists() {
                match tokio::fs::read(&cover_webp).await {
                    Ok(bytes) => {
                        return ([(header::CONTENT_TYPE, "image/webp")], bytes).into_response();
                    }
                    Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                }
            }

            // Fallback to JPG
            let cover_jpg = parent.join("cover.jpg");
            if cover_jpg.exists() {
                match tokio::fs::read(&cover_jpg).await {
                    Ok(bytes) => {
                        return ([(header::CONTENT_TYPE, "image/jpeg")], bytes).into_response();
                    }
                    Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                }
            }
        }
    }

    StatusCode::NOT_FOUND.into_response()
}
