use std::collections::HashMap;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::order::Order;

pub struct SalesDbContext {
    orders: Mutex<HashMap<Uuid, Order>>,
}

impl SalesDbContext {
    #[must_use]
    pub fn new() -> Self {
        Self {
            orders: Mutex::new(HashMap::new()),
        }
    }
    pub async fn add_order(&self, order: Order) {
        self.orders.lock().await.insert(order.order_id, order);
    }
}

impl Default for SalesDbContext {
    fn default() -> Self {
        Self::new()
    }
}
