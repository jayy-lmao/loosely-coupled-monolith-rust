use axum::Router;
use std::sync::Arc;

use common::EventDispatcher;

pub fn register_listeners(dispatcher: &mut EventDispatcher) {
    shipping::configure_services::register_listeners(dispatcher);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut dispatcher = EventDispatcher::new();

    register_listeners(&mut dispatcher);

    let app = Router::new()
        .merge(sales::endpoints::init_router(Arc::new(dispatcher)))
        .merge(shipping::endpoints::init_router());

    let listener = tokio::net::TcpListener::bind("localhost:8000")
        .await
        .expect("Could not start server");

    println!("Starting service on 0.0.0.0:8000");

    axum::serve(listener, app).await.expect("Server crashed");
}
