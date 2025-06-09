use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bdk::{Wallet, database::MemoryDatabase};

#[derive(Deserialize)]
pub struct WalletRequest {
    pub mnemonic: Option<String>,
}

#[derive(Serialize)]
pub struct WalletResponse {
    pub wallet_id: String,
    pub descriptor: String,
    pub mnemonic: Option<String>,
}

pub type WalletStore = Arc<Mutex<HashMap<String, Wallet<MemoryDatabase>>>>;