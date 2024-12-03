use tokio::sync::Mutex;

pub struct ShippingService {
    pub count: Mutex<u64>,
}

impl ShippingService {
    pub fn new() -> ShippingService {
        ShippingService {
            count: Mutex::new(0),
        }
    }
}

impl Default for ShippingService {
    fn default() -> Self {
        Self::new()
    }
}
