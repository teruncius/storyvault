pub mod audiobook_user_history;
pub mod audiobook_user_progress;

pub use audiobook_user_history::AudiobookUserHistoryProjector;
pub use audiobook_user_progress::AudiobookUserProgressProjector;

use crate::events::Event;
use std::future::Future;

/// Trait for projectors that process events and update projections
pub trait Projector: Send + Sync {
    /// Check if this projector handles the given event
    fn handles(&self, event: &Event) -> bool;

    /// Process an event and update the corresponding projection
    fn project<'a>(
        &'a self,
        event: &'a Event,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), ProjectionError>> + Send + 'a>>;
}

#[derive(Debug)]
pub struct ProjectionError {
    message: String,
}

impl ProjectionError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::error::Error for ProjectionError {}

impl std::fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Projection error: {}", self.message)
    }
}

impl From<sqlx::Error> for ProjectionError {
    fn from(error: sqlx::Error) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}
