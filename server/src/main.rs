mod routes;
mod wallet;

use axum::{Router};
use routes::health::health_router;
use std::net::SocketAddr;
use tracing_subscriber;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);
    let app = Router::new()
        .merge(health_router())
        .layer(cors)
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
