use std::sync::Arc;

use async_channel::{unbounded, Receiver, Sender};
use sales_contracts::OrderPlaced;

#[derive(Clone)]
pub enum Message {
    OrderPlaced(OrderPlaced),
}

#[async_trait::async_trait]
pub trait MessageHandler<T> {
    async fn handle(&self, message: T) -> Result<(), String>;
}

pub struct MessageBus {
    pub sender: Arc<Sender<Message>>,
    pub receiver: Arc<Receiver<Message>>,
}

impl From<OrderPlaced> for Message {
    fn from(order_placed: OrderPlaced) -> Self {
        Message::OrderPlaced(order_placed)
    }
}

impl MessageBus {
    #[must_use]
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        MessageBus {
            sender: Arc::new(sender),
            receiver: Arc::new(receiver),
        }
    }

    pub async fn publish(&self, event: impl Into<Message>) {
        let _ = self.sender.send(event.into()).await;
    }
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new()
    }
}
