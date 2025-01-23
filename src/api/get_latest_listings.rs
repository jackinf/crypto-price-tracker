use crate::api::common::Status;
use crate::api::API_URL_BASE;
use crate::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct QueryParams {
    start: u32,
    limit: u32,
    convert: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub status: Status,
    pub data: Vec<CryptoCurrency>,
}

#[derive(Debug, Deserialize)]
pub struct CryptoCurrency {
    // pub id: u32,
    pub name: String,
    pub symbol: String,
    pub quote: Quote,
}

#[derive(Debug, Deserialize)]
pub struct Quote {
    #[serde(rename = "USD")]
    pub usd: Usd,
}

#[derive(Debug, Deserialize)]
pub struct Usd {
    pub price: f64,
}

pub async fn get_latest_listings(api_key: &str, base_url: &str) -> Result<ApiResponse> {
    let query_params = QueryParams {
        start: 1,
        limit: 5000,
        convert: "USD".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .get(base_url.to_owned() + "/v1/cryptocurrency/listings/latest")
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

    let crypto_data: ApiResponse = response.json().await?;

    d!("{crypto_data:?}");

    Ok(crypto_data)
}
