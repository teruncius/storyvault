use crate::events::{Event, EventPayload};
use crate::projections::{Projection, Projector};
use sqlx::SqlitePool;
use std::future::Future;
use uuid::Uuid;

/// Data struct representing the latest progress for a user's audiobook
pub struct ProjectionData {
    pub audiobook_id: Uuid,
    pub user_id: Uuid,
    pub last_position_seconds: u64,
}

/// Projector that processes AudiobookProgress events and updates user progress
pub struct AudiobookUserProgressProjector {
    projection: AudiobookUserProgressProjection,
}

impl AudiobookUserProgressProjector {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            projection: AudiobookUserProgressProjection::new(pool),
        }
    }
}

impl Projector for AudiobookUserProgressProjector {
    fn handles(&self, event: &Event) -> bool {
        matches!(event.payload, EventPayload::AudiobookProgress(_))
    }

    fn project<'a>(
        &'a self,
        event: &'a Event,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), sqlx::Error>> + Send + 'a>> {
        Box::pin(async move {
            let EventPayload::AudiobookProgress(payload) = &event.payload;

            let data = ProjectionData {
                audiobook_id: payload.audiobook_id,
                user_id: payload.user_id,
                last_position_seconds: payload.position_seconds,
            };

            self.projection.save(&data).await?;
            Ok(())
        })
    }
}

/// Projection that manages saving user progress data to the database
pub struct AudiobookUserProgressProjection {
    pool: SqlitePool,
}

impl AudiobookUserProgressProjection {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get the latest position for a user's audiobook
    pub async fn get_position(
        &self,
        audiobook_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<u64>, sqlx::Error> {
        let result: Option<(u64,)> = sqlx::query_as(
            r#"
            SELECT last_position_seconds
            FROM audiobook_user_progress
            WHERE audiobook_id = ? AND user_id = ?
            "#,
        )
        .bind(audiobook_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|(position,)| position))
    }

    /// Get all positions for a user, returning a map of audiobook_id -> position_seconds
    pub async fn get_all_positions(
        &self,
        user_id: Uuid,
    ) -> Result<std::collections::HashMap<Uuid, u64>, sqlx::Error> {
        let results: Vec<(Uuid, u64)> = sqlx::query_as(
            r#"
            SELECT audiobook_id, last_position_seconds
            FROM audiobook_user_progress
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let mut positions = std::collections::HashMap::new();
        for (audiobook_id, position) in results {
            positions.insert(audiobook_id, position);
        }

        Ok(positions)
    }
}

impl Projection<ProjectionData> for AudiobookUserProgressProjection {
    async fn save(&self, data: &ProjectionData) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO audiobook_user_progress (audiobook_id, user_id, last_position_seconds)
            VALUES (?, ?, ?)
            ON CONFLICT(audiobook_id, user_id) 
            DO UPDATE SET last_position_seconds = excluded.last_position_seconds
            "#,
        )
        .bind(data.audiobook_id)
        .bind(data.user_id)
        .bind(data.last_position_seconds as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
