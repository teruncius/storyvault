use super::Event;
use sqlx::SqlitePool;

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
        .bind(event.event_id)
        .bind(topic)
        .bind(payload_json)
        .bind(event.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
