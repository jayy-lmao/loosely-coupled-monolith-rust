use axum::Router;
use shipping::service::ShippingService;
use std::sync::Arc;

use common::EventDispatcher;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let dispatcher = EventDispatcher::new();

    let shipping_service = Arc::new(ShippingService::new(dispatcher.clone()));
    shipping::configure_services::register_listeners(shipping_service, dispatcher.clone()).await;

    let app = Router::new()
        .merge(sales::endpoints::init_router(dispatcher))
        .merge(shipping::endpoints::init_router());

    let listener = tokio::net::TcpListener::bind("localhost:8000")
        .await
        .expect("Could not start server");

    println!("Starting service on 0.0.0.0:8000");

    axum::serve(listener, app).await.expect("Server crashed");
}
