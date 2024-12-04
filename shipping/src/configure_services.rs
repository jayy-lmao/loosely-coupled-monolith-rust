use crate::{create_shipping_label::handle_order_placed, service::ShippingService};
use common::EventDispatcher;
use std::sync::Arc;

pub async fn register_listeners(state: Arc<ShippingService>, dispatcher: EventDispatcher) {
    dispatcher
        .add_event_listener(state, handle_order_placed)
        .await;
}
