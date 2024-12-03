use crate::service::ShippingService;
use sales_contracts::OrderPlaced;
use std::sync::Arc;

pub async fn create_shipping_label(state: Arc<ShippingService>, order_placed: OrderPlaced) {
    let mut count = state.count.lock().await;
    *count += 1;

    println!(
        "placed order has been received: {order_id}. {count} orders in total",
        order_id = order_placed.order_id,
        count = count
    );
}
