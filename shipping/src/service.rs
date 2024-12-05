use common::EventDispatcher;
use tokio::sync::Mutex;

use crate::shipping_db_context::ShippingDbContext;

pub(crate) struct ShippingService {
    pub(crate) count: Mutex<u64>,
    pub(crate) shipping_db_context: ShippingDbContext,
    pub(crate) dispatcher: EventDispatcher,
}

impl ShippingService {
    pub(crate) fn new(dispatcher: EventDispatcher) -> Self {
        Self {
            count: Mutex::new(0),
            shipping_db_context: ShippingDbContext::new(),
            dispatcher,
        }
    }
}
