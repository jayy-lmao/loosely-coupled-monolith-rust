use uuid::Uuid;

pub struct Order {
    pub order_id: Uuid,
    pub customer_id: Uuid,
}
