use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct OrderBilled {
    pub order_id: Uuid,
}
