use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct ShippingLabel {
    pub order_id: Uuid,
    pub order_at: DateTime<Utc>,
    pub cancelled: bool,
    pub order_placed: bool,
    pub order_billed: bool,
    pub order_complete: bool,
}

impl Default for ShippingLabel {
    fn default() -> Self {
        Self {
            order_id: Uuid::new_v4(),
            order_at: Utc::now(),
            cancelled: false,
            order_placed: true,
            order_billed: false,
            order_complete: false,
        }
    }
}
