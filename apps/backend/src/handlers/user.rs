use axum::{Json, extract::State};
use serde::Serialize;

use crate::User;

#[derive(Serialize)]
pub struct UserResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub avatar_url: String,
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
    let users = state.users.read().unwrap();
    let mut response: Vec<UserResponse> = users.values().map(|u| u.into()).collect();
    response.sort_by(|a, b| a.email.cmp(&b.email));
    Json(response)
}
