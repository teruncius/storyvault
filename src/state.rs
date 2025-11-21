use std::sync::{Arc, RwLock};
use crate::{Audiobook, Config};

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
