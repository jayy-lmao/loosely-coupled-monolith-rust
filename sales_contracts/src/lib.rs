pub mod order_placed;
pub use order_placed::*;

#[derive(Clone)]
pub enum SalesContracts {
    OrderPlaced(OrderPlaced),
}

impl From<OrderPlaced> for SalesContracts {
    fn from(order_placed: OrderPlaced) -> Self {
        SalesContracts::OrderPlaced(order_placed)
    }
}
