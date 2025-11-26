pub mod listener;
pub mod queue;
pub mod store;

pub use listener::EventBus;
pub use queue::EventQueue;

use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProgressType {
    Play,
    Pause,
    Stop,
    Seek,
}

impl Display for ProgressType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressType::Play => write!(f, "PLAY"),
            ProgressType::Pause => write!(f, "PAUSE"),
            ProgressType::Stop => write!(f, "STOP"),
            ProgressType::Seek => write!(f, "SEEK"),
        }
    }
}

impl std::str::FromStr for ProgressType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PLAY" => Ok(ProgressType::Play),
            "PAUSE" => Ok(ProgressType::Pause),
            "STOP" => Ok(ProgressType::Stop),
            "SEEK" => Ok(ProgressType::Seek),
            _ => Err(format!("Invalid event type: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudiobookProgressPayload {
    pub audiobook_id: Uuid,
    pub user_id: Uuid,
    pub event_type: ProgressType,
    pub position_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventPayload {
    AudiobookProgress(AudiobookProgressPayload),
}

impl EventPayload {
    pub fn topic(&self) -> &'static str {
        match self {
            EventPayload::AudiobookProgress(_) => "audiobook.progress",
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub created_at: DateTime<Utc>,
    pub event_id: Uuid,
    pub payload: EventPayload,
}
