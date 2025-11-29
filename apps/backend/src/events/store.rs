use super::Event;
use sqlx::SqlitePool;

pub struct EventStore {
    pool: SqlitePool,
}

impl EventStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn record_event(&self, event: &Event) -> Result<(), Box<dyn std::error::Error>> {
        let topic = event.payload.topic();
        let payload_json = event.payload.to_json()?;

        println!("Recording event: {:#?}", event);

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

        let event: Event = sqlx::query_as(r#"SELECT * FROM events WHERE event_id = ?"#)
            .bind(event.event_id)
            .fetch_one(&self.pool)
            .await?;

        println!("Loaded stored event: {:#?}", event);

        Ok(())
    }
}
