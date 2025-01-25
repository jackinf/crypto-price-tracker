mod api;
mod cli;
mod config;
mod error;
mod prelude;

use crate::api::{BinanceApi, CoinmarketcapApi, CryptoApi, KrakenApi};
use crate::config::Config;
use crate::prelude::*;
use tokio::join;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = Config::load()?;
    let args = cli::Args::parse_args(&config); // TODO: use args - extract pairs

    let binance = BinanceApi::new();
    let kraken = KrakenApi::new(config.kraken_api_key, config.kraken_private_key);
    let coinmarketcap = CoinmarketcapApi::new(&config.cmc_base_url, &config.cmc_api_key);

    let (binance_price, kraken_price, coinmarketcap_price) = join!(
        binance.get_latest_quotes("BTCUSDT"),
        kraken.get_latest_quotes("BTC/USDT"),
        coinmarketcap.get_latest_quotes("BTC")
    );

    match (binance_price, kraken_price, coinmarketcap_price) {
        (Ok(b_price), Ok(k_price), Ok(cmc_price)) => {
            println!("Binance BTC price: {}", b_price);
            println!("Kraken BTC price: {}", k_price);
            println!("Coinmarketcap BTC price: {}", cmc_price);
        }
        (Err(b_err), Err(k_err), Err(cmc_price)) => {
            println!("Binance error: {}", b_err);
            println!("Kraken error: {}", k_err);
            println!("Coinmarketcap error: {}", cmc_price);
        }
        (Err(b_err), _, _) => println!("Binance error: {}", b_err),
        (_, Err(k_err), _) => println!("Kraken error: {}", k_err),
        (_, _, Err(cmc_err)) => println!("Coinmarketcap error: {}", cmc_err),
    }

    Ok(())
}
