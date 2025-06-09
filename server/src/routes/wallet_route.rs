use crate::handler::wallet_handler::AppState;
use crate::handler::wallet_handler::{create_wallet_handler, get_new_address};
use axum::{
    Router,
    routing::{get, post},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn wallet_router() -> Router {
    Router::new()
        .route("/wallets", post(create_wallet_handler))
        .route("/wallets/:ids/address", get(get_new_address))
        .with_state(AppState {
            wallet_store: Arc::new(Mutex::new(HashMap::new())),
        })
}
