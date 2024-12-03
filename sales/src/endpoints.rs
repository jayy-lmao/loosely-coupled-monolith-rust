use std::sync::Arc;

use axum::{extract::State, routing::post, Router};
use common::EventDispatcher;
use sales_contracts::OrderPlaced;
use uuid::Uuid;

use crate::{order::OrderStatus, sales_db_context::SalesDbContext, Order};

pub fn init_router(dispatcher: Arc<EventDispatcher>) -> Router {
    let sales_db = SalesDbContext::new();

    Router::new()
        .route("/sales", post(create_order))
        .with_state((dispatcher, Arc::new(sales_db)))
}

async fn create_order(
    State((dispatcher, sales_db)): State<(Arc<EventDispatcher>, Arc<SalesDbContext>)>,
) -> String {
    let order = Order {
        order_id: Uuid::new_v4(),
        customer_id: Uuid::new_v4(),
        status: OrderStatus::Pending,
    };

    let order_placed = OrderPlaced {
        order_id: order.order_id,
    };

    sales_db.add_order(order).await;
    dispatcher.trigger(order_placed.clone()).await;

    format!("Order {} has been placed", order_placed.order_id)
}
