use crate::prelude::*;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub default_symbols: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv().ok(); // Load .env file if it exists

        let api_key = env::var("COINMARKETCAP_API").map_err(|_| {
            Error::Config("COINMARKETCAP_API environment variable not set".to_string())
        })?;

        // Load default symbols, perhaps from an environment variable or config file
        let default_symbols = env::var("DEFAULT_SYMBOLS")
            .unwrap_or_else(|_| "BTC,ETH".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Ok(Config {
            api_key,
            default_symbols,
        })
    }
}
