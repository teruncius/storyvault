use super::{Event, EventPayload};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct EventStore {
    pool: SqlitePool,
}

impl EventStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn record_event(&self, event: &Event) -> Result<(), sqlx::Error> {
        let topic = event.payload.topic();
        let payload_json = event
            .payload
            .to_json()
            .map_err(|e| sqlx::Error::Encode(Box::new(e)))?;

        sqlx::query(
            r#"
            INSERT INTO events (event_id, topic, payload, created_at)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(event.event_id.to_string())
        .bind(topic)
        .bind(payload_json)
        .bind(event.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_latest_position(
        &self,
        audiobook_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<i64>, sqlx::Error> {
        let result: Option<(String,)> = sqlx::query_as(
            r#"
            SELECT payload
            FROM events
            WHERE topic = 'audiobook.progress'
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some((payload_json,)) = result
            && let Ok(EventPayload::AudiobookProgress(payload)) =
                serde_json::from_str::<EventPayload>(&payload_json)
            && payload.audiobook_id == audiobook_id
            && payload.user_id == user_id
        {
            return Ok(Some(payload.position_seconds));
        }

        Ok(None)
    }
}
