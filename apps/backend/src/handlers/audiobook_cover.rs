use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;

#[derive(Deserialize)]
pub struct CoverParams {
    width: Option<u32>,
}

pub async fn get_audiobook_cover(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<CoverParams>,
) -> Response {
    let book_path = {
        let books = state.audiobooks.read().unwrap();
        books.get(&id).map(|b| b.path.clone())
    };

    if let Some(path) = book_path
        && let Some(parent) = path.parent()
    {
        // Identify source image
        let (source_path, content_type) = if parent.join("cover.webp").exists() {
            (parent.join("cover.webp"), "image/webp")
        } else if parent.join("cover.jpg").exists() {
            (parent.join("cover.jpg"), "image/jpeg")
        } else {
            return StatusCode::NOT_FOUND.into_response();
        };

        // Handle resizing if width is requested
        if let Some(width) = params.width
            && let Some(dir_name) = parent.file_name().and_then(|n| n.to_str())
        {
            let cache_dir = state.config.vault.join("tmp").join(dir_name);
            let cache_filename = format!("cover_{}.webp", width);
            let cache_path = cache_dir.join(&cache_filename);

            // Serve from cache if exists
            if cache_path.exists()
                && let Ok(bytes) = tokio::fs::read(&cache_path).await
            {
                return ([(header::CONTENT_TYPE, "image/webp")], bytes).into_response();
            }

            // Resize and cache
            let source_path_clone = source_path.clone();
            let cache_path_clone = cache_path.clone();
            let cache_dir_clone = cache_dir.clone();

            let resize_result = tokio::task::spawn_blocking(move || {
                let img = image::open(&source_path_clone)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                let resized = img.resize(width, width, image::imageops::FilterType::Lanczos3);

                if !cache_dir_clone.exists() {
                    std::fs::create_dir_all(&cache_dir_clone)
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                }

                resized
                    .save(&cache_path_clone)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                Ok::<_, StatusCode>(())
            })
            .await;

            if let Ok(Ok(_)) = resize_result
                && let Ok(bytes) = tokio::fs::read(&cache_path).await
            {
                return ([(header::CONTENT_TYPE, "image/webp")], bytes).into_response();
            }
        }

        // Serve original image
        match tokio::fs::read(&source_path).await {
            Ok(bytes) => {
                return ([(header::CONTENT_TYPE, content_type)], bytes).into_response();
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    StatusCode::NOT_FOUND.into_response()
}
