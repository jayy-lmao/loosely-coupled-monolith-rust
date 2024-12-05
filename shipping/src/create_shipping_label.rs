use crate::{service::ShippingService, shipping_label::ShippingLabel};
use axum::async_trait;
use billing_contracts::OrderBilled;
use sales_contracts::OrderPlaced;
use shipping_contracts::CreateShippingLabel;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub(crate) trait ShippingServiceHandlers {
    async fn process_order(self, order_id: Uuid);
    async fn handle_order_placed(self, order_placed: OrderPlaced);
    async fn handle_order_billed(self, order_billed: OrderBilled);
}

#[async_trait]
impl ShippingServiceHandlers for Arc<ShippingService> {
    async fn process_order(self, order_id: Uuid) {
        let maybe_label = self.shipping_db_context.get_shipping_label(order_id).await;
        if let Some(label) = maybe_label {
            let mut count = self.count.lock().await;
            *count += 1;

            if label.order_placed && label.order_billed {
                self.dispatcher
                    .trigger(CreateShippingLabel { order_id })
                    .await;
            }
        }
    }

    async fn handle_order_placed(self, order_placed: OrderPlaced) {
        let order_id = order_placed.order_id;

        let mut label = self
            .shipping_db_context
            .get_shipping_label(order_id)
            .await
            .unwrap_or(ShippingLabel {
                order_id,
                ..Default::default()
            });

        label.order_placed = true;

        self.shipping_db_context.upsert_shipping_label(label).await;

        self.clone().process_order(order_id).await;

        println!(
            "order has been placed: {order_id}. ",
            order_id = order_placed.order_id,
        );
    }

    async fn handle_order_billed(self, order_billed: OrderBilled) {
        let order_id = order_billed.order_id;

        let mut label = self
            .shipping_db_context
            .get_shipping_label(order_id)
            .await
            .unwrap_or(ShippingLabel {
                order_id,
                ..Default::default()
            });

        label.order_billed = true;

        self.shipping_db_context.upsert_shipping_label(label).await;

        self.clone().process_order(order_id).await;

        println!(
            "order has been billed: {order_id}. ",
            order_id = order_billed.order_id,
        );
    }
}
