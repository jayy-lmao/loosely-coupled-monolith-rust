use std::collections::HashMap;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::shipping_label::ShippingLabel;

pub struct ShippingDbContext {
    shipping_labels: Mutex<HashMap<Uuid, ShippingLabel>>,
}

impl ShippingDbContext {
    #[must_use]
    pub fn new() -> Self {
        Self {
            shipping_labels: Mutex::new(HashMap::new()),
        }
    }
    pub async fn get_shipping_label(&self, order_id: Uuid) -> Option<ShippingLabel> {
        self.shipping_labels.lock().await.get(&order_id).cloned()
    }

    pub async fn upsert_shipping_label(&self, shipping_label: ShippingLabel) {
        self.shipping_labels
            .lock()
            .await
            .insert(shipping_label.order_id, shipping_label);
    }
}

impl Default for ShippingDbContext {
    fn default() -> Self {
        Self::new()
    }
}
