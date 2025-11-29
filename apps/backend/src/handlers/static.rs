use axum::{
    http::{StatusCode, Uri, header},
    response::IntoResponse,
};
use include_dir::{Dir, include_dir};

static FRONTEND_DIR: Dir<'_> = include_dir!("apps/frontend/dist");

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() {
        let file = FRONTEND_DIR.get_file("index.html").unwrap();
        return ([(header::CONTENT_TYPE, "text/html")], file.contents()).into_response();
    }

    match FRONTEND_DIR.get_file(path) {
        Some(file) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], file.contents()).into_response()
        }
        None => match FRONTEND_DIR.get_file("index.html") {
            Some(index_file) => {
                ([(header::CONTENT_TYPE, "text/html")], index_file.contents()).into_response()
            }
            None => StatusCode::NOT_FOUND.into_response(),
        },
    }
}
