use crate::prelude::*;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub cmc_base_url: String,
    pub cmc_api_key: String,
    pub binance_api_key: String,
    pub binance_private_key: String,
    pub kraken_api_key: String,
    pub kraken_private_key: String,
    pub default_symbols: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv().ok(); // Load .env file if it exists

        let cmc_api_key = env::var("CPT_COINMARKETCAP_API")
            .map_err(|_| Error::EnvNotSet("CPT_COINMARKETCAP_API".to_string()))?;

        let cmc_base_url = env::var("CPT_COINMARKETCAP_BASE_URL")
            .map_err(|_| Error::EnvNotSet("CPT_COINMARKETCAP_API".to_string()))?;

        let binance_api_key = env::var("CPT_BINANCE_API_KEY")
            .map_err(|_| Error::EnvNotSet("CPT_BINANCE_API_KEY".to_string()))?;

        let binance_private_key = env::var("CPT_BINANCE_PRIVATE_KEY")
            .map_err(|_| Error::EnvNotSet("CPT_BINANCE_PRIVATE_KEY".to_string()))?;

        let kraken_api_key = env::var("CPT_KRAKEN_API_KEY")
            .map_err(|_| Error::EnvNotSet("CPT_KRAKEN_API_KEY".to_string()))?;

        let kraken_private_key = env::var("CPT_KRAKEN_PRIVATE_KEY")
            .map_err(|_| Error::EnvNotSet("CPT_KRAKEN_PRIVATE_KEY".to_string()))?;

        // Load default symbols, perhaps from an environment variable or config file
        let default_symbols = env::var("DEFAULT_SYMBOLS")
            .unwrap_or_else(|_| "BTC,ETH".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Ok(Config {
            cmc_base_url,
            cmc_api_key,
            binance_api_key,
            binance_private_key,
            kraken_api_key,
            kraken_private_key,
            default_symbols,
        })
    }
}
