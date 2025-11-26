use tokio::sync::mpsc;

use super::Event;

/// Event queue that holds events to be processed
#[derive(Clone)]
pub struct EventQueue {
    sender: mpsc::UnboundedSender<Event>,
}

impl EventQueue {
    /// Create a new event queue and return both the queue and receiver
    pub fn new() -> (Self, mpsc::UnboundedReceiver<Event>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (Self { sender }, receiver)
    }

    /// Enqueue an event for processing
    pub fn enqueue(&self, event: Event) -> Result<(), String> {
        self.sender
            .send(event)
            .map_err(|e| format!("Failed to enqueue event: {}", e))
    }
}
