use crate::{
    AppState, Config,
    auth::auth_middleware,
    frontend::static_handler,
    handlers::{
        get_audiobook, get_audiobook_cover, get_problems, get_users, health_check, index,
        list_audiobooks, login, logout, me, register, set_audiobook_position, stream_audiobook,
    },
};
use axum::{
    Router,
    http::{
        Method,
        header::{AUTHORIZATION, CONTENT_TYPE, COOKIE},
    },
    middleware,
    routing::{get, post, put},
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub fn build_app(state: AppState, config: &Config) -> Router {
    let cors = build_cors(config);

    // Public routes (no authentication required)
    let public_router = Router::new()
        .route("/api", get(index))
        .route("/api/health", get(health_check))
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register));

    // Protected routes (authentication required)
    let protected_router = Router::new()
        .route("/api/audiobook", get(list_audiobooks))
        .route("/api/audiobook/{id}", get(get_audiobook))
        .route("/api/audiobook/{id}/cover", get(get_audiobook_cover))
        .route("/api/audiobook/{id}/position", put(set_audiobook_position))
        .route("/api/audiobook/{id}/stream", get(stream_audiobook))
        .route("/api/user", get(get_users))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/me", get(me))
        .route("/api/problem", get(get_problems))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(public_router)
        .merge(protected_router)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
        .fallback(static_handler)
}

fn build_cors(config: &Config) -> CorsLayer {
    let mut cors = CorsLayer::new();

    let origins: Vec<_> = config
        .cors
        .allowed_origins
        .iter()
        .filter_map(|origin| axum::http::HeaderValue::from_str(origin).ok())
        .collect();

    cors = if origins.is_empty() {
        cors.allow_origin(Any)
    } else {
        cors.allow_origin(origins)
    };

    cors.allow_methods([
        Method::CONNECT,
        Method::DELETE,
        Method::GET,
        Method::HEAD,
        Method::OPTIONS,
        Method::PATCH,
        Method::POST,
        Method::PUT,
        Method::TRACE,
    ])
    .allow_headers(vec![AUTHORIZATION, CONTENT_TYPE, COOKIE])
    .allow_credentials(true)
}
