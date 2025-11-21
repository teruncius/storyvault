use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::Config;

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

#[derive(Clone)]
pub struct AppState {
    pub audiobooks: Arc<RwLock<Vec<Audiobook>>>,
    pub config: Config,
}

pub fn build_state(config: &Config) -> AppState {
    AppState {
        audiobooks: Arc::new(RwLock::new(Vec::new())),
        config: config.clone(),
    }
}
