use axum::{
    Router,
    routing::{get, put, post},
};
use clap::Parser;
use std::path::PathBuf;

mod state;
pub use state::{AppState, build_state};

mod scan;
pub use scan::{initial_scan, setup_watcher};

mod config;
pub use config::Config;

mod handlers;
pub use handlers::{
    health_check,
    get_audiobook_cover,
    get_audiobook_position,
    index,
    list_audiobooks,
    set_audiobook_position,
    stream_audiobook,
    get_users,
    login,
    logout,
    me,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "storyvault.yaml")]
    config: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Audiobook {
    id: Uuid,
    title: String,
    author: String,
    #[serde(default)]
    year: Option<u32>,
    #[serde(skip)]
    path: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Load configuration
    let config = Config::from_file(&args.config).unwrap_or_else(|e| {
        eprintln!("Failed to load config from {:?}: {}", args.config, e);
        std::process::exit(1);
    });

    let shared_state = build_state(&config);

    // Initial scan
    let audiobooks_dir = config.audiobooks_dir();
    initial_scan(&audiobooks_dir, &shared_state);

    // Setup file watcher
    let _watcher = setup_watcher(config.vault.clone(), shared_state.clone());

    // Build our application
    let app = build_app(shared_state);

    // Run it
    build_server(app, &config).await;
}

fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
        .route("/audiobook", get(list_audiobooks))
        .route("/audiobook/{id}/cover", get(get_audiobook_cover))
        .route("/audiobook/{id}/position", get(get_audiobook_position))
        .route("/audiobook/{id}/position", put(set_audiobook_position))
        .route("/audiobook/{id}/stream", get(stream_audiobook))
        .route("/user", get(get_users))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
        .with_state(state)
}

async fn build_server(app: Router, config: &Config) {
    let addr = config.socket_addr();
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
