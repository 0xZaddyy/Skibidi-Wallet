use crate::handler::wallet_handler::create_wallet_handler;
use axum::{Router, routing::post};

pub fn wallet_router() -> Router {
    Router::new().route("/wallets", post(create_wallet_handler))
}
