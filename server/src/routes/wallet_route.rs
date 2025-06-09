use crate::handler::wallet_handler::{create_wallet_handler, get_new_address};
use crate::handler::wallet_handler::AppState;
use axum::{Router, routing::{post, get}};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;


pub fn wallet_router() -> Router {
    Router::new()
    .route("/wallets", post(create_wallet_handler))
    .route("x", get(get_new_address))
    .with_state(AppState {
        wallet_store: Arc::new(Mutex::new(HashMap::new())),
})
}
