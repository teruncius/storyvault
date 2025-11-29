use crate::state::Session;
use crate::user::{User, UserRepository};
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
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

pub async fn login(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<LoginRequest>,
) -> Result<Response, StatusCode> {
    let repository = UserRepository::new(&state.db_pool);

    let user = match repository.get_users_by_email(&payload.email).await {
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
        Ok(user) => user,
    };

    // Verify password
    if !bcrypt::verify(&payload.password, &user.password_hash).unwrap_or(false) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Create session
    let session_id = Uuid::new_v4();
    let expires_at = Utc::now() + Duration::hours(SESSION_DURATION_HOURS);

    // Store session
    {
        let mut sessions = state
            .sessions
            .write()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        sessions.insert(
            session_id,
            Session {
                user_id: user.id,
                expires_at,
            },
        );
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
        HeaderValue::from_str(&cookie_value).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

    Ok(response)
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

pub async fn register(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<RegisterRequest>,
) -> Result<Response, StatusCode> {
    let password_hash = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let user = User {
        id: Uuid::new_v4(),
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        password_hash,
        avatar_url: None,
    };

    let repository = UserRepository::new(&state.db_pool);
    match repository.save_user(user).await {
        Ok(_) => Ok(StatusCode::OK.into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
