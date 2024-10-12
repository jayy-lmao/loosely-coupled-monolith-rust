use axum::async_trait;
use common::{Message, MessageHandler};

use crate::create_shipping_label::CreateShippingLabel;

pub struct ShippingMessageHandler;

#[async_trait]
impl MessageHandler<Message> for ShippingMessageHandler {
    async fn handle(&self, message: Message) -> Result<(), String> {
        match message {
            Message::OrderPlaced(order_placed) => {
                CreateShippingLabel::new().handle(order_placed).await?;
            }
        };
        Ok(())
    }
}