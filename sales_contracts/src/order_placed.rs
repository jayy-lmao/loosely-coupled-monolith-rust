use uuid::Uuid;

#[derive(Clone)]
pub struct OrderPlaced {
    pub order_id: Uuid,
}
