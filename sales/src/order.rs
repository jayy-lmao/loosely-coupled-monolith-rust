use uuid::Uuid;

#[derive(Clone, Copy)]
pub enum OrderStatus {
    Pending,
    ReadyToShip,
    Shipped,
    Cancelled,
}

#[derive(Clone)]
pub struct Order {
    pub order_id: Uuid,
    pub customer_id: Uuid,
    pub status: OrderStatus,
}
