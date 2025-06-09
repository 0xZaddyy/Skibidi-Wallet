use crate::model::wallet_model::{WalletRequest, WalletResponse, WalletStore};
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::keys::{DerivableKey, ExtendedKey, GeneratableKey, bip39::Mnemonic};
use bdk::wallet::{AddressIndex, Wallet};
use nanoid::nanoid;
use serde::Serialize;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub wallet_store: WalletStore,
}

#[derive(Serialize)]
pub struct AddressResponse {
    pub address: String,
}

pub async fn create_wallet_handler(Json(req): Json<WalletRequest>) -> Json<WalletResponse> {
    let (mnemonic, is_new) = match req.mnemonic {
        Some(m) => {
            let result = Mnemonic::parse(&m);
            match result {
                Ok(parsed) => {
                    info!("Wallet import from mnemonic successful");
                    (parsed, false)
                }
                Err(e) => {
                    info!("Wallet import from mnemonic failed: {}", e);
                    panic!("Invalid mnemonic: {}", e);
                }
            }
        }
        None => {
            let generated = <Mnemonic as GeneratableKey<bdk::miniscript::Segwitv0>>::generate((
                bdk::keys::bip39::WordCount::Words12,
                bdk::keys::bip39::Language::English,
            ));
            info!("New wallet created successfully");
            (generated.unwrap().into_key(), true)
        }
    };
    let xkey: ExtendedKey = mnemonic.clone().into_extended_key().unwrap();
    let xprv = xkey.into_xprv(Network::Testnet).unwrap();
    let descriptor = format!("wpkh({})", xprv.to_string());
    let _wallet = Wallet::new(
        &descriptor,
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    )
    .unwrap();
    let wallet_id = nanoid!();
    Json(WalletResponse {
        wallet_id,
        descriptor,
        mnemonic: if is_new {
            Some(mnemonic.to_string())
        } else {
            None
        },
    })
}

pub async fn get_new_address(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let store = state.wallet_store.lock().unwrap();

    let wallet = match store.get(&id) {
        Some(w) => w,
        None => return Err((axum::http::StatusCode::NOT_FOUND, "Wallet not found")),
    };

    let address = wallet.get_address(AddressIndex::New).unwrap();
    Ok(Json(AddressResponse {
        address: address.to_string(),
    }))
}
