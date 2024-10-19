use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use common::{Message, MessageBus};
use sales_contracts::SalesContracts;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_create_order() {
    // Setup
    let message_bus = Arc::new(MessageBus::new());
    let app = sales::endpoints::init_router(message_bus.clone());
    let request = Request::builder()
        .method("POST")
        .uri("/sales")
        .body(Body::empty())
        .unwrap();

    // Act
    let response = app.oneshot(request).await.unwrap();

    let response_status = response.status();
    let body = axum::body::to_bytes(response.into_body(), 2000)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let message = message_bus
        .receiver
        .recv()
        .await
        .expect("No message was received on the message bus");

    // Assert
    assert_eq!(response_status, StatusCode::OK);
    assert!(body_str.contains("Order"));
    assert!(
        matches!(message, Message::Sales(SalesContracts::OrderPlaced(_))),
        "Received unexpected message type"
    );
    match message {
        Message::Sales(SalesContracts::OrderPlaced(order_placed)) => {
            assert!(body_str.contains(&order_placed.order_id.to_string()));
        }
    }
}
