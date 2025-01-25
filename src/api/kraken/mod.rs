use std::collections::HashMap;
use crate::api::CryptoApi;
use crate::prelude::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod auth;
mod common;
mod get_latest_ticker;

pub struct KrakenApi {
    base_url: String,
    api_key: String,
    secret_key: String,
}

impl KrakenApi {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            base_url: "https://api.kraken.com".to_string(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
        }
    }
}

/*

*/
#[derive(Serialize, Deserialize)]
struct BalanceResult {
    #[serde(rename = "DOT")]
    pub dot: String,
    #[serde(rename = "ETHW")]
    pub ethw: String,
    #[serde(rename = "LUNA")]
    pub luna: String,
    #[serde(rename = "LUNA2")]
    pub luna2: String,
    #[serde(rename = "MATIC")]
    pub matic: String,
    #[serde(rename = "PEPE")]
    pub pepe: String,
    #[serde(rename = "POL")]
    pub pol: String,
    #[serde(rename = "SOL")]
    pub sol: String,
    #[serde(rename = "TREMP")]
    pub tremp: String,
    #[serde(rename = "USDT")]
    pub usdt: String,
    #[serde(rename = "XETH")]
    pub xeth: String,
    #[serde(rename = "XLTC")]
    pub xltc: String,
    #[serde(rename = "XXBT")]
    pub xxbt: String,
    #[serde(rename = "XXDG")]
    pub xxdg: String,
    #[serde(rename = "XXMR")]
    pub xxmr: String,
    #[serde(rename = "ZEUR")]
    pub zeur: String,
    #[serde(rename = "ZUSD")]
    pub zusd: String,
}

#[derive(Serialize, Deserialize)]
struct BalanceResultRoot {
    pub error: Vec<String>,
    pub result: BalanceResult,
}

#[async_trait]
impl CryptoApi for KrakenApi {
    async fn get_latest_quotes(&self, symbol: &str) -> Result<f64> {
        let url = f!("{}/0/public/Ticker?pair={}", self.base_url, symbol);
        let resp = reqwest::get(&url).await?;
        let json: serde_json::Value = resp.json().await?;

        let price = json["result"][symbol]["c"][0]
            .as_str()
            .ok_or_else(|| Error::ApiPayloadParseError)?
            .parse::<f64>()?;

        Ok(price)
    }

    async fn check_balance(&self) -> Result<f64> {
        let url = "/0/private/Balance";
        let full_url = f!("{}{}", self.base_url, url);
        let auth = auth::Auth::new(self.api_key.to_string(), self.secret_key.to_string())?;
        let nonce = auth.nonce();
        let signature = auth.sign([("nonce", nonce)], url, nonce)?;
        let params = HashMap::from([("nonce", nonce.to_string())]);

        let response = reqwest::Client::new()
            .post(full_url)
            .header("API-Key", auth.api_key_header)
            .header("API-Sign", signature)
            .form(&params)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;

        d!("{json:?}");

        let balance: BalanceResultRoot = serde_json::from_value(json)?;
        Ok(balance.result.usdt.parse::<f64>()?)
    }

    async fn get_symbols(&self) -> Result<Vec<String>> {
        Ok(vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_latest_quotes() {
        let api = KrakenApi::new("".to_string(), "".to_string());
        let res = api.get_latest_quotes("BTCUSDT").await.unwrap();
        assert!(res > 0.0);
    }

    #[tokio::test]
    async fn test_check_balance() {
        let api = KrakenApi::new("".to_string(), "".to_string());
        let res = api.check_balance().await.unwrap();
        assert!(res > 0.0);
    }

    #[tokio::test]
    async fn test_get_symbols() {
        let api = KrakenApi::new("".to_string(), "".to_string());
        let res = api.get_symbols().await.unwrap();
        assert_eq!(res, vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]);
    }
}
