use crate::{service::ShippingService, shipping_label::ShippingLabel};
use sales_contracts::OrderPlaced;
use std::sync::Arc;
use uuid::Uuid;

async fn process_order(state: Arc<ShippingService>, order_id: Uuid) {
    let maybe_label = state.shipping_db_context.get_shipping_label(order_id).await;
    if let Some(label) = maybe_label {
        if label.order_placed && label.order_billed {
            // Create shipping label
        }
    }
}

pub(crate) async fn handle_order_placed(state: Arc<ShippingService>, order_placed: OrderPlaced) {
    let mut count = state.count.lock().await;
    *count += 1;

    let mut label = state
        .shipping_db_context
        .get_shipping_label(order_placed.order_id)
        .await
        .unwrap_or(ShippingLabel {
            order_id: order_placed.order_id,
            ..Default::default()
        });

    label.order_placed = true;

    state.shipping_db_context.upsert_shipping_label(label).await;

    println!(
        "placed order has been received: {order_id}. {count} orders in total",
        order_id = order_placed.order_id,
        count = count
    );
}
