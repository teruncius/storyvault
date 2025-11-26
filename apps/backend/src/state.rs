use crate::Config;
use crate::events::EventQueue;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanProblemType {
    MissingIndexYaml,
    MissingAudioFile,
    InvalidYamlFormat,
    UnableToExtractDuration,
    MissingCover,
    InvalidDataFormat,
    MissingStorageDirectory,
    FailedToReadFile,
    FailedToReadDirectory,
    FailedToReadDirectoryEntry,
    ScanFailed,
    RescanFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProblem {
    pub path: PathBuf,
    pub problem_type: ScanProblemType,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Audiobook {
    pub id: Uuid,
    pub title: String,
    #[serde(alias = "author", deserialize_with = "deserialize_authors")]
    pub authors: Vec<String>,
    pub year: u32,
    #[serde(skip)]
    pub path: PathBuf,
    #[serde(skip)]
    pub duration_seconds: Option<u64>,
}

fn deserialize_authors<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum AuthorsOrAuthor {
        Authors(Vec<String>),
        Author(String),
    }

    match AuthorsOrAuthor::deserialize(deserializer)? {
        AuthorsOrAuthor::Authors(authors) => Ok(authors),
        AuthorsOrAuthor::Author(author) => Ok(vec![author]),
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub avatar_url: String,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct AppState {
    pub audiobooks: Arc<RwLock<HashMap<Uuid, Audiobook>>>,
    pub config: Config,
    pub sessions: Arc<RwLock<HashMap<Uuid, Session>>>,
    pub users: Arc<RwLock<HashMap<Uuid, User>>>,
    pub db_pool: SqlitePool,
    pub event_queue: EventQueue,
    pub scan_problems: Arc<RwLock<Vec<ScanProblem>>>,
}

pub fn build_state(config: &Config, db_pool: SqlitePool, event_queue: EventQueue) -> AppState {
    let mut users = HashMap::new();

    let alice_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let bob_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();

    users.insert(
        alice_id,
        User {
            id: alice_id,
            first_name: "Alice".into(),
            last_name: "Smith".into(),
            email: "alice@example.com".into(),
            password_hash: bcrypt::hash("password123", bcrypt::DEFAULT_COST)
                .unwrap_or_else(|_| "$2b$12$dummy_hash_for_alice".to_string()),
            avatar_url: "https://example.com/avatars/alice.png".into(),
        },
    );

    users.insert(
        bob_id,
        User {
            id: bob_id,
            first_name: "Bob".into(),
            last_name: "Jones".into(),
            email: "bob@example.com".into(),
            password_hash: bcrypt::hash("password456", bcrypt::DEFAULT_COST)
                .unwrap_or_else(|_| "$2b$12$dummy_hash_for_bob".to_string()),
            avatar_url: "https://example.com/avatars/bob.png".into(),
        },
    );

    AppState {
        audiobooks: Arc::new(RwLock::new(HashMap::new())),
        config: config.clone(),
        sessions: Arc::new(RwLock::new(HashMap::new())),
        users: Arc::new(RwLock::new(users)),
        db_pool,
        event_queue,
        scan_problems: Arc::new(RwLock::new(Vec::new())),
    }
}
