mod common;
mod get_latest_listings;
mod get_latest_listings_test;
mod get_latest_quotes;
mod get_latest_quotes_test;

pub const API_URL_BASE: &str = "https://sandbox-api.coinmarketcap.com";

pub use get_latest_listings::*;
pub use get_latest_quotes::*;
