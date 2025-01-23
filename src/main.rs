mod error;
mod prelude;
mod api;

use std::env;
use clap::Parser;
// use clap_derive::Parser;
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};

pub use error::{Error, Result};
use crate::prelude::*;

#[derive(Serialize, Deserialize)]
struct CryptoQueryParams {
    start: u32,
    limit: u32,
    convert: String,
    // symbol: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "BTC,ETH")]
    symbols: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    dotenv().ok();
    let args = Args::parse();

    let coinmarketcap_api = env::var("COINMARKETCAP_API")
        .expect("COINMARKETCAP_API must be set");
    let url = "https://sandbox-api.coinmarketcap.com/v1/cryptocurrency/listings/latest";
    let query_params = CryptoQueryParams {
        start: 1,
        limit: 5000,
        convert: "USD".to_string(),
        // symbol: args.symbols.clone()
    };

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("X-CMC_PRO_API_KEY", &coinmarketcap_api)
        .header("Accept", "application/json")
        .header("Accept-Encoding", "deflate, gzip")
        .query(&query_params)
        .send()
        .await
        .map_err(|e| Error::Something)?;

    let json_response: serde_json::Value = response.json().await
        .map_err(|e| Error::Something)?;

    d!("{json_response:?}");

    println!("{coinmarketcap_api:?}");

    Ok(())
}
