use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    Play,
    Pause,
    Stop,
    Seek,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Play => write!(f, "PLAY"),
            EventType::Pause => write!(f, "PAUSE"),
            EventType::Stop => write!(f, "STOP"),
            EventType::Seek => write!(f, "SEEK"),
        }
    }
}

impl std::str::FromStr for EventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PLAY" => Ok(EventType::Play),
            "PAUSE" => Ok(EventType::Pause),
            "STOP" => Ok(EventType::Stop),
            "SEEK" => Ok(EventType::Seek),
            _ => Err(format!("Invalid event type: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AudiobookEvent {
    pub event_id: Uuid,
    pub audiobook_id: Uuid,
    pub user_id: Uuid,
    pub event_type: String,
    pub position_seconds: i64,
    pub created_at: DateTime<Utc>,
}

pub struct EventStore {
    pool: SqlitePool,
}

impl EventStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn record_event(
        &self,
        audiobook_id: Uuid,
        user_id: Uuid,
        event_type: EventType,
        position_seconds: i64,
    ) -> Result<AudiobookEvent, sqlx::Error> {
        let event_id = Uuid::new_v4();
        let created_at = Utc::now();
        let event_type_str = event_type.to_string();

        sqlx::query(
            r#"
            INSERT INTO events (event_id, audiobook_id, user_id, event_type, position_seconds, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(event_id.to_string())
        .bind(audiobook_id.to_string())
        .bind(user_id.to_string())
        .bind(&event_type_str)
        .bind(position_seconds)
        .bind(created_at)
        .execute(&self.pool)
        .await?;

        Ok(AudiobookEvent {
            event_id,
            audiobook_id,
            user_id,
            event_type: event_type_str,
            position_seconds,
            created_at,
        })
    }

    pub async fn get_latest_position(
        &self,
        audiobook_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<i64>, sqlx::Error> {
        let result: Option<(i64,)> = sqlx::query_as(
            r#"
            SELECT position_seconds
            FROM events
            WHERE audiobook_id = ? AND user_id = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(audiobook_id.to_string())
        .bind(user_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.0))
    }
}
