use axum::{extract::State, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub avatar_url: String,
}

pub async fn get_users(State(_state): State<crate::AppState>) -> Json<Vec<UserResponse>> {
    let users = vec![
        UserResponse {
            first_name: "Alice".into(),
            last_name: "Smith".into(),
            email: "alice@example.com".into(),
            avatar_url: "https://example.com/avatars/alice.png".into(),
        },
        UserResponse {
            first_name: "Bob".into(),
            last_name: "Jones".into(),
            email: "bob@example.com".into(),
            avatar_url: "https://example.com/avatars/bob.png".into(),
        },
    ];
    Json(users)
}
