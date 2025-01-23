use crate::model::*;
use crate::prelude::*;
use reqwest;
use serde::{Serialize};

const API_URL: &str = "https://sandbox-api.coinmarketcap.com/v1/cryptocurrency/listings/latest";

#[derive(Debug, Serialize)]
struct CryptoQueryParams {
    start: u32,
    limit: u32,
    convert: String,
    // symbol: String,
}

pub async fn get_latest_listings(
    api_key: &str,
    symbols: &str,
) -> Result<Vec<CryptoCurrency>> {
    let query_params = CryptoQueryParams {
        start: 1,
        limit: 5000,
        convert: "USD".to_string(),
        // symbol: symbols.to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .get(API_URL)
        .header("X-CMC_PRO_API_KEY", api_key)
        .header("Accept", "application/json")
        .header("Accept-Encoding", "deflate, gzip")
        .query(&query_params)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        return Err(Error::Api(format!(
            "API request failed with status {}: {}",
            status, text
        )));
    }

    let crypto_data: CryptoData = response.json().await?;
    // let crypto_data: serde_json::Value = response.json().await?;

    d!("{crypto_data:?}");

    Ok(crypto_data.data)
}