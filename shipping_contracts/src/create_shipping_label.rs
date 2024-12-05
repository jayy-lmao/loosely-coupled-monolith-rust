use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CreateShippingLabel {
    pub order_id: Uuid,
}
