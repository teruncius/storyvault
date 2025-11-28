use axum::Router;
use clap::Parser;
use std::path::PathBuf;

mod state;
pub use state::{AppState, Audiobook, ScanProblem, ScanProblemType, Session, User, build_state};

mod scan;
pub use scan::{initial_scan, setup_watcher};

mod config;
pub use config::{Config, SESSION_COOKIE_NAME, SESSION_DURATION_HOURS};

mod app;
mod auth;
mod db;
mod events;
mod frontend;
mod handlers;
mod iso8601;
mod projections;
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

    // Initialize database
    let db_pool = db::init_db(&config.database.url).await.unwrap_or_else(|e| {
        eprintln!("Failed to initialize database: {}", e);
        std::process::exit(1);
    });

    // Create event queue and bus
    let (event_queue, receiver) = events::EventQueue::new();
    let event_bus = events::EventBus::new(receiver, db_pool.clone());

    // Spawn background task to process events
    tokio::spawn(async move {
        event_bus.start().await;
    });

    let shared_state = build_state(&config, db_pool, event_queue);

    // Initial scan
    let audiobooks_dir = config.audiobooks_dir();
    initial_scan(&audiobooks_dir, &shared_state);

    // Setup file watcher
    let _watcher = setup_watcher(config.vault.clone(), shared_state.clone());

    // Build our application
    let app = build_app(shared_state, &config);

    // Run it
    build_server(app, &config).await;
}

async fn build_server(app: Router, config: &Config) {
    let addr = config.socket_addr();
    println!("Started server on http://{}/", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
