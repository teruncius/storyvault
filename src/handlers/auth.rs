use axum::{extract::{State, Json}, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(State(_state): State<AppState>, Json(payload): Json<LoginRequest>) -> (StatusCode, Json<LoginResponse>) {
    let _ = payload;
    let resp = LoginResponse {
        token: "dummy-token-123".to_string(),
    };
    (StatusCode::OK, Json(resp))
}

pub async fn logout(State(_state): State<AppState>) -> StatusCode {
    StatusCode::OK
}

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: String,
}

pub async fn me(State(_state): State<AppState>) -> Json<MeResponse> {
    let resp = MeResponse {
        email: "alice@example.com".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Smith".to_string(),
        avatar_url: "https://example.com/avatars/alice.png".to_string(),
    };
    Json(resp)
}
