use std::sync::Arc;

use async_channel::{unbounded, Receiver, Sender};
use sales_contracts::{OrderPlaced, SalesContracts};

#[derive(Clone)]
pub enum Message {
    Sales(SalesContracts),
}

#[async_trait::async_trait]
pub trait MessageHandler<T> {
    async fn handle(&self, message: T) -> Result<(), String>;
}

pub struct MessageBus {
    pub sender: Arc<Sender<Message>>,
    pub receiver: Arc<Receiver<Message>>,
}

impl From<SalesContracts> for Message {
    fn from(sales_msg: SalesContracts) -> Self {
        Message::Sales(sales_msg)
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
