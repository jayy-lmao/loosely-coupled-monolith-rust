use axum::Router;

use common::EventDispatcher;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let dispatcher = EventDispatcher::new();

    shipping::configure_services::register_listeners(dispatcher.clone()).await;

    let app = Router::new()
        .merge(sales::endpoints::init_router(dispatcher))
        .merge(shipping::endpoints::init_router());

    let listener = tokio::net::TcpListener::bind("localhost:8000")
        .await
        .expect("Could not start server");

    println!("Starting service on 0.0.0.0:8000");

    axum::serve(listener, app).await.expect("Server crashed");
}
