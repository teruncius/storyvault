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
    pub source: Option<String>,
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
pub struct Session {
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct AppState {
    pub audiobooks: Arc<RwLock<HashMap<Uuid, Audiobook>>>,
    pub config: Config,
    pub sessions: Arc<RwLock<HashMap<Uuid, Session>>>,
    pub db_pool: SqlitePool,
    pub event_queue: EventQueue,
    pub scan_problems: Arc<RwLock<Vec<ScanProblem>>>,
}

pub fn build_state(config: &Config, db_pool: SqlitePool, event_queue: EventQueue) -> AppState {
    AppState {
        audiobooks: Arc::new(RwLock::new(HashMap::new())),
        config: config.clone(),
        sessions: Arc::new(RwLock::new(HashMap::new())),
        db_pool,
        event_queue,
        scan_problems: Arc::new(RwLock::new(Vec::new())),
    }
}
