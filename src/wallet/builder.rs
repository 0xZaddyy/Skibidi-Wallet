use bdk_wallet::Wallet;
use bdk_wallet::WalletBuilder;
use bdk_chain::bitcoin::Network;
use std::path::PathBuf;


pub async fn init_wallet(
    descriptor: &str,
    change_descriptor: Option<&str>,
    db_path: PathBuf,
) -> anyhow::Result<Wallet> {
    let wallet = WalletBuilder::new()
        .set_descriptor(descriptor)?
        .set_change_descriptor_opt(change_descriptor)?
        .set_network(Network::Testnet) // or .set_network(Network::Bitcoin)
        .set_database_path(db_path)
        .build()
        .await?;

    Ok(wallet)
}
