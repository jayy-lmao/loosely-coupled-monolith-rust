pub mod order_placed;
pub use order_placed::*;

pub enum SalesContracts {
    OrderPlaced(OrderPlaced),
}
