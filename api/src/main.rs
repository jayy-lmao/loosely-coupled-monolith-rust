use axum::Router;
use std::sync::Arc;
use worker::Worker;

use common::MessageBus;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("Starting service on 0.0.0.0:8000");
    let message_bus = Arc::new(MessageBus::new());
    let mut worker = Worker::new(message_bus.clone());

    worker.register(Box::new(shipping::message_handler::ShippingMessageHandler));

    let app = Router::new()
        .merge(sales::endpoints::init_router(message_bus))
        .merge(shipping::endpoints::init_router());

    let listener = tokio::net::TcpListener::bind("localhost:8000")
        .await
        .unwrap();

    tokio::spawn(async move {
        worker.run().await;
    });
    axum::serve(listener, app).await.unwrap();
}
