use std::sync::Arc;

use axum::{extract::State, routing::post, Router};
use common::MessageBus;
use sales_contracts::OrderPlaced;
use uuid::Uuid;

pub fn init_router(publisher: Arc<MessageBus>) -> Router {
    Router::new()
        .route("/sales", post(create_order))
        .with_state(publisher)
}

async fn create_order(State(message_bus): State<Arc<MessageBus>>) -> String {
    let order_placed = OrderPlaced {
        order_id: Uuid::new_v4(),
    };
    message_bus.publish(order_placed.clone()).await;

    format!("Order {} has been placed", order_placed.order_id)
}
