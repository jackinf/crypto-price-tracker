use crypto_price_tracker::{
    config::Config,
    prelude::*
};
use crypto_price_tracker::api::KrakenApi;
use crypto_price_tracker::api::CryptoApi;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let config = Config::load()?;
    let kraken = KrakenApi::new(config.kraken_api_key, config.kraken_private_key);
    let balance = kraken.check_balance().await?;

    f!("Balance: {}", balance);

    Ok(())
}