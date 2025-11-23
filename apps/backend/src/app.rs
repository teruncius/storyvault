use crate::{
    AppState, Config,
    auth::auth_middleware,
    handlers::{
        get_audiobook_cover, get_audiobook_position, get_users, health_check, index,
        list_audiobooks, login, logout, me, set_audiobook_position, stream_audiobook,
    },
};
use axum::{
    Router,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware,
    routing::{get, post, put},
};
use tower_http::cors::{Any, CorsLayer};

pub fn build_app(state: AppState, config: &Config) -> Router {
    let cors = build_cors(config);

    // Public routes (no authentication required)
    let public_router = Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
        .route("/auth/login", post(login));

    // Protected routes (authentication required)
    let protected_router = Router::new()
        .route("/audiobook", get(list_audiobooks))
        .route("/audiobook/{id}/cover", get(get_audiobook_cover))
        .route("/audiobook/{id}/position", get(get_audiobook_position))
        .route("/audiobook/{id}/position", put(set_audiobook_position))
        .route("/audiobook/{id}/stream", get(stream_audiobook))
        .route("/user", get(get_users))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(public_router)
        .merge(protected_router)
        .layer(cors)
        .with_state(state)
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

    cors.allow_methods(Any)
        .allow_headers(vec![AUTHORIZATION, CONTENT_TYPE])
}
