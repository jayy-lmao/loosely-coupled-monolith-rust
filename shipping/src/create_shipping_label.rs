use axum::async_trait;
use common::MessageHandler;
use sales_contracts::OrderPlaced;

pub struct CreateShippingLabel;
impl CreateShippingLabel {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for CreateShippingLabel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MessageHandler<OrderPlaced> for CreateShippingLabel {
    async fn handle(&self, order: OrderPlaced) -> Result<(), String> {
        println!("placed order has been received: {}", order.order_id);
        Ok(())
    }
}
