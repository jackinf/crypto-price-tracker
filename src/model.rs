use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CryptoData {
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