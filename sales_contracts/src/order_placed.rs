use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct OrderPlaced {
    pub order_id: Uuid,
}
