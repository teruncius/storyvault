use axum::{
    Router,
    routing::{get, put, post},
};
use crate::{AppState, handlers::{
    health_check,
    get_audiobook_cover,
    get_audiobook_position,
    index,
    list_audiobooks,
    set_audiobook_position,
    stream_audiobook,
    get_users,
    login,
    logout,
    me,
}};

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
        .route("/audiobook", get(list_audiobooks))
        .route("/audiobook/{id}/cover", get(get_audiobook_cover))
        .route("/audiobook/{id}/position", get(get_audiobook_position))
        .route("/audiobook/{id}/position", put(set_audiobook_position))
        .route("/audiobook/{id}/stream", get(stream_audiobook))
        .route("/user", get(get_users))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
        .with_state(state)
}

