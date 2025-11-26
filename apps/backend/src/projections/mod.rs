pub mod audiobook_user_progress;

pub use audiobook_user_progress::AudiobookUserProgressProjector;

use crate::events::Event;
use std::future::Future;

/// Trait for projections that represent a materialized view of events
/// The projection holds the database connection and saves data structs
pub trait Projection<D>: Send + Sync {
    /// Save or update the projection in the database with the given data
    async fn save(&self, data: &D) -> Result<(), sqlx::Error>;
}

/// Trait for projectors that process events and update projections
pub trait Projector: Send + Sync {
    /// Check if this projector handles the given event
    fn handles(&self, event: &Event) -> bool;

    /// Process an event and update the corresponding projection
    fn project<'a>(
        &'a self,
        event: &'a Event,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), sqlx::Error>> + Send + 'a>>;
}
