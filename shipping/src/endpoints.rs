use axum::{routing::get, Router};

pub fn init_router() -> Router {
    Router::new().route("/shipping", get(get_shipping))
}

async fn get_shipping() -> String {
    String::from("Hello Shipping!")
}
