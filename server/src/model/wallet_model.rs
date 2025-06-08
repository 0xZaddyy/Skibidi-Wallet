use serde::{Deserialize, Serialize};

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
