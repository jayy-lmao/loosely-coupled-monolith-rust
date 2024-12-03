use crate::{create_shipping_label::create_shipping_label, service::ShippingService};
use common::EventDispatcher;
use std::sync::Arc;

pub fn register_listeners(dispatcher: &mut EventDispatcher) {
    let state = Arc::new(ShippingService::new());
    dispatcher.add_event_listener(state, create_shipping_label);
}
