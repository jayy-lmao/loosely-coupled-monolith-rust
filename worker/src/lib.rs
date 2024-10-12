use std::sync::Arc;

use common::Message;
use common::MessageBus;
use common::MessageHandler;

pub struct Worker {
    message_bus: Arc<MessageBus>,
    handlers: Vec<Box<dyn MessageHandler<Message> + Send + Sync>>,
}

impl Worker {
    #[must_use]
    pub fn new(message_bus: Arc<MessageBus>) -> Self {
        Self {
            message_bus,
            handlers: vec![],
        }
    }

    pub fn register(&mut self, handler: Box<dyn MessageHandler<Message> + Send + Sync>) {
        self.handlers.push(handler);
    }

    pub async fn run(&self) {
        while let Ok(msg) = self.message_bus.receiver.recv().await {
            for handler in &self.handlers {
                handler.handle(msg.clone()).await;
            }
        }
    }
}
