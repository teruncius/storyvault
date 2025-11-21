use crate::Config;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Audiobook {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    #[serde(default)]
    pub year: Option<u32>,
    #[serde(skip)]
    pub path: PathBuf,
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
    pub audiobooks: Arc<RwLock<Vec<Audiobook>>>,
    pub config: Config,
    pub sessions: Arc<RwLock<HashMap<Uuid, Session>>>,
    pub users: Arc<RwLock<Vec<User>>>,
}

pub fn build_state(config: &Config) -> AppState {
    let users = vec![
        User {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            first_name: "Alice".into(),
            last_name: "Smith".into(),
            email: "alice@example.com".into(),
            password_hash: bcrypt::hash("password123", bcrypt::DEFAULT_COST)
                .unwrap_or_else(|_| "$2b$12$dummy_hash_for_alice".to_string()),
            avatar_url: "https://example.com/avatars/alice.png".into(),
        },
        User {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            first_name: "Bob".into(),
            last_name: "Jones".into(),
            email: "bob@example.com".into(),
            password_hash: bcrypt::hash("password456", bcrypt::DEFAULT_COST)
                .unwrap_or_else(|_| "$2b$12$dummy_hash_for_bob".to_string()),
            avatar_url: "https://example.com/avatars/bob.png".into(),
        },
    ];

    AppState {
        audiobooks: Arc::new(RwLock::new(Vec::new())),
        config: config.clone(),
        sessions: Arc::new(RwLock::new(HashMap::new())),
        users: Arc::new(RwLock::new(users)),
    }
}
