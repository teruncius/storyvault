use crate::{AppState, SESSION_COOKIE_NAME, user::User, user::UserRepository};
use axum::{
    extract::{FromRequestParts, Request, State},
    http::{StatusCode, request::Parts},
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use uuid::Uuid;

/// Extract the authenticated user from request extensions
/// This should be used in handlers that require authentication
#[derive(Clone)]
pub struct AuthenticatedUser(pub User);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

/// Middleware to validate session cookie and load user
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    // Get headers from request
    let headers = request.headers();

    // Get session ID from cookie
    let session_id = match headers.get(axum::http::header::COOKIE) {
        Some(cookie_header) => match cookie_header.to_str() {
            Ok(cookie_str) => cookie_str.split(';').find_map(|cookie| {
                let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
                if parts.len() == 2 && parts[0] == SESSION_COOKIE_NAME {
                    Uuid::parse_str(parts[1]).ok()
                } else {
                    None
                }
            }),
            Err(_) => None,
        },
        None => None,
    };

    let session_id = match session_id {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // Find session
    let session = {
        let sessions = match state.sessions.read() {
            Ok(sessions) => sessions,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        match sessions.get(&session_id) {
            Some(session) => session.clone(),
            None => return StatusCode::UNAUTHORIZED.into_response(),
        }
    };

    // Check if session is expired
    if session.expires_at < Utc::now() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    // Find user by ID
    let repository = UserRepository::new(&state.db_pool);
    let user = repository.get_users_by_id(session.user_id).await;

    let user = match user {
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
        Ok(user) => user,
    };

    // Add user to request extensions
    request.extensions_mut().insert(AuthenticatedUser(user));

    next.run(request).await
}
