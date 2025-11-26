use super::store::EventStore;
use super::{Event, EventPayload};
use sqlx::SqlitePool;
use tokio::sync::mpsc;

/// Event listener that processes events from the queue
pub struct EventListener {
    receiver: mpsc::UnboundedReceiver<Event>,
    event_store: EventStore,
}

impl EventListener {
    /// Create a new event listener
    pub fn new(receiver: mpsc::UnboundedReceiver<Event>, db_pool: SqlitePool) -> Self {
        Self {
            receiver,
            event_store: EventStore::new(db_pool),
        }
    }

    /// Start listening for events and processing them
    pub async fn start(mut self) {
        println!("Event listener started");

        while let Some(event) = self.receiver.recv().await {
            if let Err(e) = self.process_event(event).await {
                eprintln!("Failed to process event: {}", e);
            }
        }

        println!("Event listener stopped");
    }

    /// Process a single event by storing it in the database
    async fn process_event(&self, event: Event) -> Result<(), String> {
        match &event.payload {
            EventPayload::AudiobookProgress(payload) => {
                println!(
                    "Processing event: {} for audiobook {} user {} at position {}",
                    event.event_id, payload.audiobook_id, payload.user_id, payload.position_seconds
                );
            }
        }

        self.event_store
            .record_event(&event)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        println!("Event {} processed successfully", event.event_id);
        Ok(())
    }
}
