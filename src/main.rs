use axum::Router;
use clap::Parser;
use std::path::PathBuf;

mod state;
pub use state::{AppState, Audiobook, Session, User, build_state};

mod scan;
pub use scan::{initial_scan, setup_watcher};

mod config;
pub use config::{Config, SESSION_COOKIE_NAME, SESSION_DURATION_HOURS};

mod app;
mod auth;
mod handlers;
pub use app::build_app;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "storyvault.yaml")]
    config: PathBuf,
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

async fn build_server(app: Router, config: &Config) {
    let addr = config.socket_addr();
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
