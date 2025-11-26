use crate::{User, auth::AuthenticatedUser};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: String,
}

impl From<&User> for MeResponse {
    fn from(user: &User) -> Self {
        MeResponse {
            id: user.id.to_string(),
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            avatar_url: user.avatar_url.clone(),
        }
    }
}

pub async fn me(AuthenticatedUser(user): AuthenticatedUser) -> axum::Json<MeResponse> {
    axum::Json(MeResponse::from(&user))
}
