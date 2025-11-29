use axum::{Json, extract::State};
use serde::Serialize;

use crate::user::{User, UserRepository};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        UserResponse {
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            avatar_url: user.avatar_url.clone(),
        }
    }
}

pub async fn get_users(State(state): State<crate::AppState>) -> Json<Vec<UserResponse>> {
    let repository = UserRepository::new(&state.db_pool);
    let users = repository.get_users().await.unwrap_or_default();

    let response: Vec<UserResponse> = users.iter().map(|u| u.into()).collect();
    Json(response)
}
