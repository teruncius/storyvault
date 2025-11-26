use crate::state::Session;
use crate::{AppState, auth::AuthenticatedUser};
use crate::{SESSION_COOKIE_NAME, SESSION_DURATION_HOURS};
use axum::{
    extract::State,
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use chrono::{Duration, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<LoginRequest>,
) -> Result<Response, StatusCode> {
    let users = state
        .users
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Find user by email
    let user = users.values().find(|u| u.email == payload.email);

    match user {
        None => Err(StatusCode::UNAUTHORIZED),
        Some(user) => {
            // Verify password
            if !bcrypt::verify(&payload.password, &user.password_hash).unwrap_or(false) {
                return Err(StatusCode::UNAUTHORIZED);
            }

            // Create session
            let session_id = Uuid::new_v4();
            let expires_at = Utc::now() + Duration::hours(SESSION_DURATION_HOURS);

            let session = Session {
                user_id: user.id,
                expires_at,
            };

            // Store session
            {
                let mut sessions = state
                    .sessions
                    .write()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                sessions.insert(session_id, session);
            }

            // Create cookie header value
            let max_age_secs = SESSION_DURATION_HOURS * 3600;
            let cookie_value = format!(
                "{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
                SESSION_COOKIE_NAME, session_id, max_age_secs
            );

            let mut response = axum::response::NoContent.into_response();

            response.headers_mut().insert(
                axum::http::header::SET_COOKIE,
                HeaderValue::from_str(&cookie_value)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            );

            Ok(response)
        }
    }
}

pub async fn logout(
    State(state): State<AppState>,
    AuthenticatedUser(_user): AuthenticatedUser,
    request: axum::extract::Request,
) -> Response {
    // Get session ID from cookie to remove it
    let session_id = request
        .headers()
        .get(axum::http::header::COOKIE)
        .and_then(|h| h.to_str().ok())
        .and_then(|cookie_str| {
            cookie_str.split(';').find_map(|cookie| {
                let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
                if parts.len() == 2 && parts[0] == SESSION_COOKIE_NAME {
                    Uuid::parse_str(parts[1]).ok()
                } else {
                    None
                }
            })
        });

    // Remove session if we found the session ID
    if let Some(session_id) = session_id {
        let mut sessions = state.sessions.write().unwrap();
        sessions.remove(&session_id);
    }

    // Remove cookie by setting it to expire immediately
    let cookie_value = format!(
        "{}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0",
        SESSION_COOKIE_NAME
    );

    let mut response = StatusCode::OK.into_response();
    response.headers_mut().insert(
        axum::http::header::SET_COOKIE,
        HeaderValue::from_str(&cookie_value).unwrap(),
    );

    response
}
