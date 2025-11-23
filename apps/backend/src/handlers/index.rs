use crate::AppState;
use axum::{Json, extract::State};
use serde::Serialize;

#[derive(Serialize)]
pub struct IndexResponse {
    pub health_url: String,
    pub auth_me_url: String,
    pub user_url: String,
    pub audiobook_url: String,
}

pub async fn index(State(state): State<AppState>) -> Json<IndexResponse> {
    let base_url = &state.config.url.base;
    let response = IndexResponse {
        health_url: format!("{}/health", base_url),
        auth_me_url: format!("{}/auth/me", base_url),
        user_url: format!("{}/user", base_url),
        audiobook_url: format!("{}/audiobook", base_url),
    };
    Json(response)
}
