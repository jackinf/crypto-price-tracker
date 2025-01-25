use crate::api::coinmarketcap::common::Status;
use crate::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct QueryParams {
    id: Option<String>,
    convert: Option<String>,
    symbol: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
    pub status: Status,
    pub data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    #[serde(flatten)]
    pub currencies: std::collections::HashMap<String, CurrencyData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrencyData {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub is_active: u32,
    pub is_fiat: Option<u32>,
    pub circulating_supply: Option<f64>,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub date_added: String,
    pub num_market_pairs: u32,
    pub cmc_rank: u32,
    pub last_updated: String,
    pub tags: Vec<String>,
    pub platform: Option<Platform>,
    pub self_reported_circulating_supply: Option<f64>,
    pub self_reported_market_cap: Option<f64>,
    pub quote: Quote,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Quote {
    #[serde(rename = "USD")]
    pub usd: Usd,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Usd {
    pub price: f64,
    pub volume_24h: f64,
    pub volume_change_24h: f64,
    pub percent_change_1h: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
    pub percent_change_30d: f64,
    pub market_cap: f64,
    pub market_cap_dominance: f64,
    pub fully_diluted_market_cap: f64,
    pub last_updated: String,
}

// Placeholder for a potential Platform struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Platform {
    // Define fields for the platform data if needed
}

pub async fn get_latest_quotes(
    api_key: &str,
    base_url: &str,
    symbols: &str,
) -> Result<ApiResponse> {
    let query_params = QueryParams {
        id: None,
        convert: Some("USD".to_string()),
        symbol: Some(symbols.to_string()),
    };

    let client = reqwest::Client::new();
    let response = client
        .get(base_url.to_owned() + "/cryptocurrency/quotes/latest")
        .header("X-CMC_PRO_API_KEY", api_key)
        .header("Accept", "application/json")
        // .header("Accept-Encoding", "deflate, gzip")
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
