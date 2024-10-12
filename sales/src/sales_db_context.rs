use std::collections::HashMap;

use uuid::Uuid;

use crate::order::Order;

pub struct SalesDbContext {
    orders: HashMap<Uuid, Order>,
}

impl SalesDbContext {
    #[must_use]
    pub fn new(orders: HashMap<Uuid, Order>) -> Self {
        Self { orders }
    }
}
