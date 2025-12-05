use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

use crate::events::{Event, EventPayload};
use crate::projections::ProjectionError;
use crate::projections::Projector;
use chrono::DateTime;
use chrono::Utc;
use sqlx::Row;

#[derive(Debug, serde::Serialize)]
pub struct ProjectionData {
    pub user_id: Uuid,
    pub audiobook_id: Uuid,
    pub accessed_at: DateTime<Utc>,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for ProjectionData {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user_id: row.get("user_id"),
            audiobook_id: row.get("audiobook_id"),
            accessed_at: row.get("accessed_at"),
        })
    }
}

pub struct AudiobookUserHistoryProjector {
    projection: AudiobookUserHistoryProjection,
}

impl AudiobookUserHistoryProjector {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            projection: AudiobookUserHistoryProjection { pool },
        }
    }
}

impl Projector for AudiobookUserHistoryProjector {
    fn handles(&self, event: &Event) -> bool {
        matches!(
            event.payload,
            EventPayload::AudiobookProgress(_) | EventPayload::ResetAudiobookProgress(_)
        )
    }

    fn project<'a>(
        &'a self,
        event: &'a Event,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), ProjectionError>> + Send + 'a>> {
        Box::pin(async move {
            match &event.payload {
                EventPayload::AudiobookProgress(payload) => {
                    let data = ProjectionData {
                        user_id: payload.user_id,
                        audiobook_id: payload.audiobook_id,
                        accessed_at: event.created_at,
                    };
                    self.projection.save(&data).await
                }
                EventPayload::ResetAudiobookProgress(payload) => {
                    self.projection
                        .delete(payload.user_id, payload.audiobook_id)
                        .await
                }
                _ => Err(ProjectionError::new("Invalid event".into())),
            }
        })
    }
}

pub struct AudiobookUserHistoryProjection {
    pool: SqlitePool,
}

impl AudiobookUserHistoryProjection {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    async fn save(&self, data: &ProjectionData) -> Result<(), ProjectionError> {
        sqlx::query(
            r#"
            INSERT INTO audiobook_user_history (user_id, audiobook_id, accessed_at)
            VALUES (?, ?, ?)
            ON CONFLICT (user_id, audiobook_id) DO UPDATE SET accessed_at = ?
            "#,
        )
        .bind(data.user_id)
        .bind(data.audiobook_id)
        .bind(data.accessed_at)
        .bind(data.accessed_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, user_id: Uuid, audiobook_id: Uuid) -> Result<(), ProjectionError> {
        sqlx::query(
            r#"
            DELETE FROM audiobook_user_history
            WHERE user_id = ? AND audiobook_id = ?
            "#,
        )
        .bind(user_id)
        .bind(audiobook_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_recent_activity(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ProjectionData>, sqlx::Error> {
        let result = sqlx::query_as(
            r#"
            SELECT *
            FROM audiobook_user_history
            WHERE user_id = ?
            ORDER BY accessed_at DESC
            LIMIT 10
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
}
