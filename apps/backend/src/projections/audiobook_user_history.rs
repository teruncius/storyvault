use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

use crate::events::{Event, EventPayload};
use crate::projections::Projector;
use crate::projections::{Projection, ProjectionError};
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
        matches!(event.payload, EventPayload::AudiobookProgress(_))
    }

    fn project<'a>(
        &'a self,
        event: &'a Event,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), ProjectionError>> + Send + 'a>> {
        Box::pin(async move {
            let EventPayload::AudiobookProgress(payload) = &event.payload else {
                return Err(ProjectionError::new("Invalid event".into()));
            };

            let data = ProjectionData {
                user_id: payload.user_id,
                audiobook_id: payload.audiobook_id,
                accessed_at: event.created_at,
            };

            self.projection.save(&data).await
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

    pub async fn get_recent_audiobooks(
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

impl Projection<ProjectionData> for AudiobookUserHistoryProjection {
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
}
