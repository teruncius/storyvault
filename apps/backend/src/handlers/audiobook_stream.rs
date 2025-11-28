use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use uuid::Uuid;

use crate::AppState;

pub async fn stream_audiobook(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> Response {
    let audio_path = {
        let books = state.audiobooks.read().unwrap();
        books.get(&id).map(|b| b.path.clone())
    };

    let Some(path) = audio_path else {
        return StatusCode::NOT_FOUND.into_response();
    };

    if !path.exists() {
        return StatusCode::NOT_FOUND.into_response();
    }

    let file_size = match tokio::fs::metadata(&path).await {
        Ok(metadata) => metadata.len(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // Parse Range header if present
    let range = headers
        .get(header::RANGE)
        .and_then(|v| v.to_str().ok())
        .and_then(parse_range);

    match range {
        Some((start, end)) => {
            let end = end.min(file_size - 1);
            let length = end - start + 1;

            let file = match tokio::fs::File::open(&path).await {
                Ok(f) => f,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            };

            let mut file = file;
            if file.seek(std::io::SeekFrom::Start(start)).await.is_err() {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }

            let mut buffer = vec![0; length as usize];
            if file.read_exact(&mut buffer).await.is_err() {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }

            let mut response_headers = HeaderMap::new();
            response_headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("audio/mpeg"));
            response_headers.insert(
                header::CONTENT_LENGTH,
                HeaderValue::from_str(&length.to_string()).unwrap(),
            );
            response_headers.insert(
                header::CONTENT_RANGE,
                HeaderValue::from_str(&format!("bytes {}-{}/{}", start, end, file_size)).unwrap(),
            );
            response_headers.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));

            (StatusCode::PARTIAL_CONTENT, response_headers, buffer).into_response()
        }
        None => {
            // No range requested, send entire file
            let bytes = match tokio::fs::read(&path).await {
                Ok(b) => b,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            };

            let mut response_headers = HeaderMap::new();
            response_headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("audio/mpeg"));
            response_headers.insert(
                header::CONTENT_LENGTH,
                HeaderValue::from_str(&file_size.to_string()).unwrap(),
            );
            response_headers.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));

            (StatusCode::OK, response_headers, bytes).into_response()
        }
    }
}

fn parse_range(range_header: &str) -> Option<(u64, u64)> {
    // Parse "bytes=start-end" format
    let range_header = range_header.strip_prefix("bytes=")?;
    let mut parts = range_header.split('-');
    let start = parts.next()?.parse::<u64>().ok()?;
    let end = parts.next().and_then(|s| {
        if s.is_empty() {
            Some(u64::MAX)
        } else {
            s.parse::<u64>().ok()
        }
    })?;

    let max_len = 2u64.pow(17);
    let end = if end.saturating_sub(start) >= max_len {
        start + max_len - 1
    } else {
        end
    };

    Some((start, end))
}
