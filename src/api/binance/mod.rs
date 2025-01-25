use crate::api::CryptoApi;
use crate::prelude::*;
use async_trait::async_trait;

mod common;
mod get_latest_ticker;

pub struct BinanceApi {
    base_url: String,
}

impl BinanceApi {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.binance.com".to_string(),
        }
    }
}

#[async_trait]
impl CryptoApi for BinanceApi {
    async fn get_latest_quotes(&self, symbol: &str) -> Result<f64> {
        let url = f!("{}/api/v3/ticker/price?symbol={}", self.base_url, symbol);
        let resp = reqwest::get(&url).await?;
        let json: serde_json::Value = resp.json().await?;

        let res = json["price"]
            .as_str()
            .ok_or_else(|| Error::ApiPayloadParseError)?
            .parse::<f64>()?;

        Ok(res)
    }

    async fn check_balance(&self) -> Result<f64> {
        todo!()
    }

    async fn get_symbols(&self) -> Result<Vec<String>> {
        Ok(vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()])
    }
}
