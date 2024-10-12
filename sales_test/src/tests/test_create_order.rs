use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use common::{Message, MessageBus};
use std::sync::Arc;
use tower::ServiceExt; // for `oneshot` method

// Test for create_order function with real MessageBus
#[tokio::test]
async fn test_create_order() {
    // Initialize the real MessageBus
    let message_bus = Arc::new(MessageBus::new());

    // Create router with the real message bus
    let app = sales::endpoints::init_router(message_bus.clone());

    // Send a request to create an order
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sales")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert the status is OK
    assert_eq!(response.status(), StatusCode::OK);

    // Check if the response contains the expected message
    let body = axum::body::to_bytes(response.into_body(), 2000)
        .await
        .unwrap();

    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert!(body_str.contains("Order"));

    // Assert that the correct message was published on the message bus
    if let Ok(message) = message_bus.receiver.recv().await {
        match message {
            Message::OrderPlaced(order_placed) => {
                // Check if the order ID in the response matches the published message
                assert!(body_str.contains(&order_placed.order_id.to_string()));
            }
        }
    } else {
        panic!("No message was received on the message bus");
    }
}
