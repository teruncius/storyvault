pub mod listener;
pub mod queue;
pub mod store;

pub use listener::EventBus;
pub use queue::EventQueue;

use sqlx::Row;
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
pub struct ResetAudiobookProgressPayload {
    pub audiobook_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestPayload {
    pub message: String,
}

#[derive(Debug)]
pub enum EventPayload {
    Test(TestPayload),
    AudiobookProgress(AudiobookProgressPayload),
    ResetAudiobookProgress(ResetAudiobookProgressPayload),
}

impl EventPayload {
    pub fn topic(&self) -> &'static str {
        match self {
            EventPayload::Test(_) => "test",
            EventPayload::AudiobookProgress(_) => "audiobook.progress",
            EventPayload::ResetAudiobookProgress(_) => "audiobook.progress_reset",
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        match self {
            EventPayload::Test(payload) => serde_json::to_string(payload),
            EventPayload::AudiobookProgress(payload) => serde_json::to_string(payload),
            EventPayload::ResetAudiobookProgress(payload) => serde_json::to_string(payload),
        }
    }

    pub fn from_parts(
        topic: &str,
        payload_json: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match topic {
            "audiobook.progress" => {
                let payload: AudiobookProgressPayload = serde_json::from_str(payload_json)?;
                Ok(EventPayload::AudiobookProgress(payload))
            }
            "audiobook.progress_reset" => {
                let payload: ResetAudiobookProgressPayload = serde_json::from_str(payload_json)?;
                Ok(EventPayload::ResetAudiobookProgress(payload))
            }
            _ => Err(format!("Unknown event topic: {}", topic).into()),
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub created_at: DateTime<Utc>,
    pub event_id: Uuid,
    pub payload: EventPayload,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Event {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            created_at: row.get("created_at"),
            event_id: row.get("event_id"),
            payload: EventPayload::from_parts(row.get("topic"), row.get("payload"))
                .map_err(sqlx::Error::Decode)?,
        })
    }
}
