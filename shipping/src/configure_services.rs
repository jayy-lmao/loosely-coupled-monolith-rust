use crate::{create_shipping_label::ShippingServiceHandlers, service::ShippingService};
use common::EventDispatcher;
use std::sync::Arc;

pub async fn register_listeners(dispatcher: EventDispatcher) {
    let shipping_service = Arc::new(ShippingService::new(dispatcher.clone()));

    dispatcher
        .add_event_listener(
            shipping_service.clone(),
            ShippingServiceHandlers::handle_order_placed,
        )
        .await
        .add_event_listener(
            shipping_service,
            ShippingServiceHandlers::handle_order_billed,
        )
        .await;
}
