use bdk_wallet::keys::bip39::{Mnemonic, Language};

pub fn generate_mnemonic() -> Mnemonic {
    Mnemonic::generate_in(Language::English, 12).unwrap()
}
