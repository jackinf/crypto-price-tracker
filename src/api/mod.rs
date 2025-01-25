use async_trait::async_trait;

mod binance;
mod coinmarketcap;
mod kraken;
use crate::prelude::*;

#[async_trait]
pub trait CryptoApi {
    async fn get_latest_quotes(&self, symbol: &str) -> Result<f64>;
    async fn check_balance(&self) -> Result<f64>;
    async fn get_symbols(&self) -> Result<Vec<String>>;
}

pub use binance::*;
pub use coinmarketcap::*;
pub use kraken::*;
