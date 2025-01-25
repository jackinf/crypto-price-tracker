mod common;
mod get_latest_listings;
mod get_latest_listings_test;
mod get_latest_quotes;
mod get_latest_quotes_test;
use crate::prelude::*;

use crate::api::CryptoApi;
use async_trait::async_trait;
pub use get_latest_listings::*;
pub use get_latest_quotes::*;

pub struct CoinmarketcapApi {
    base_url: String,
    api_key: String,
}

impl CoinmarketcapApi {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

#[async_trait]
impl CryptoApi for CoinmarketcapApi {
    async fn get_latest_quotes(&self, symbol: &str) -> Result<f64> {
        let res = get_latest_quotes::get_latest_quotes(&self.api_key, &self.base_url, symbol).await;
        match res {
            Ok(response) => {
                let quote = response
                    .data
                    .currencies
                    .values()
                    .nth(0)
                    .ok_or(Error::ApiPayloadParseError)?;
                Ok(quote.quote.usd.price)
            }
            Err(e) => Err(e),
        }
    }

    async fn check_balance(&self) -> Result<f64> {
        todo!()
    }

    async fn get_symbols(&self) -> Result<Vec<String>> {
        Ok(vec!["BTC".to_string(), "ETH".to_string()])
    }
}
