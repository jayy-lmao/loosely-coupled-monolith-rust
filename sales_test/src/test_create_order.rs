use pretty_assertions::assert_eq;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use common::EventDispatcher;
use sales_contracts::OrderPlaced;
use tower::ServiceExt;

#[tokio::test]
async fn test_create_order() {
    let dispatcher = EventDispatcher::new();
    let captured_ord: Arc<Mutex<Option<OrderPlaced>>> = Arc::new(Mutex::new(None));

    let captured_events_clone = captured_ord.clone();
    let state = Arc::new(());

    dispatcher
        .add_event_listener(state, move |_, order: OrderPlaced| {
            let captured_order_clone = captured_events_clone.clone();
            async move {
                let mut event = captured_order_clone.lock().await;
                *event = Some(order);
            }
        })
        .await;

    let app = sales::endpoints::init_router(dispatcher);
    let request = Request::builder()
        .method("POST")
        .uri("/sales")
        .body(Body::empty())
        .unwrap();

    // Act
    let response = app.oneshot(request).await.unwrap();

    let response_status = response.status();
    let body = axum::body::to_bytes(response.into_body(), 10_000)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Assert

    // Response
    assert_eq!(response_status, StatusCode::OK);
    assert!(body_str.contains("Order"));

    // Event
    let event = captured_ord.lock().await.to_owned();
    assert!(event.is_some());
    assert!(!event.unwrap().order_id.to_string().is_empty());
}
