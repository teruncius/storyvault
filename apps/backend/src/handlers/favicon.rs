use crate::AppState;
use axum::extract::State;
use axum::http::header;
use axum::response::{IntoResponse, Response};

pub async fn get_favicon(State(_state): State<AppState>) -> Response {
    let bytes = include_bytes!("../../assets/favicon.webp");

    ([(header::CONTENT_TYPE, "image/webp")], bytes).into_response()
}
