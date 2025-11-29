use super::Event;
use super::store::EventStore;
use crate::projections::Projector;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tracing::{error, info};

/// Event bus that processes events from the queue
pub struct EventBus {
    receiver: mpsc::UnboundedReceiver<Event>,
    event_store: EventStore,
    projectors: Vec<Box<dyn Projector>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new(receiver: mpsc::UnboundedReceiver<Event>, db_pool: SqlitePool) -> Self {
        let projectors: Vec<Box<dyn Projector>> = vec![Box::new(
            crate::projections::AudiobookUserProgressProjector::new(db_pool.clone()),
        )];

        Self {
            receiver,
            event_store: EventStore::new(db_pool.clone()),
            projectors,
        }
    }

    /// Start listening for events and processing them
    pub async fn start(mut self) {
        info!("Event bus started");

        while let Some(event) = self.receiver.recv().await {
            if let Err(e) = self.process_event(&event).await {
                error!("Failed to process event: {}", e);
            }
        }

        info!("Event bus stopped");
    }

    /// Process a single event by storing it in the database and updating projections
    async fn process_event(&self, event: &Event) -> Result<(), String> {
        info!(
            "Processing event: {} topic: {}",
            event.event_id,
            event.payload.topic()
        );

        // Store the event in the event store
        self.event_store
            .record_event(event)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        // Process projections
        for projector in &self.projectors {
            if projector.handles(event)
                && let Err(e) = projector.project(event).await
            {
                error!("Failed to project event {}: {}", event.event_id, e);
            }
        }

        info!("Event {} processed successfully", event.event_id);
        Ok(())
    }
}
