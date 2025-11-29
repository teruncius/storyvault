use axum::Router;
use clap::Parser;
use std::path::PathBuf;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod state;
pub use state::{AppState, Audiobook, ScanProblem, ScanProblemType, Session, build_state};

mod scan;
pub use scan::build_watcher;

mod config;
pub use config::{Config, SESSION_COOKIE_NAME, SESSION_DURATION_HOURS};

mod app;
mod auth;
mod db;
mod events;
mod handlers;
mod iso8601;
mod projections;
mod user;
pub use app::build_app;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "storyvault.yaml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match run().await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error: {}", e);
            Err(e)
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();

    let args = Args::parse();

    // Load configuration
    let config = Config::from_file(&args.config)?;

    // Initialize database
    let db_pool = db::init_db(&config.database.url).await?;
    // Execute database migrations
    db::migrate(&db_pool).await?;

    // Create event queue and bus
    let (event_queue, receiver) = events::EventQueue::new();
    let event_bus = events::EventBus::new(receiver, db_pool.clone());

    // Spawn background task to process events
    tokio::spawn(async move {
        event_bus.start().await;
    });

    let shared_state = build_state(&config, db_pool, event_queue);
    let _watcher = build_watcher(config.vault.clone(), shared_state.clone());
    let app = build_app(shared_state, &config);
    build_server(app, &config).await;

    Ok(())
}

fn setup_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "storyvault=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn build_server(app: Router, config: &Config) {
    let addr = config.socket_addr();
    info!("Started server on http://{}/", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
