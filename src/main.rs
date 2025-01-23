mod api;
mod cli;
mod config;
mod error;
mod prelude;

use crate::api::API_URL_BASE;
use crate::config::Config;
use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let config = Config::load()?;
    let args = cli::Args::parse_args(&config);

    let listings = api::get_latest_listings(&config.api_key, API_URL_BASE).await?;
    d!("{listings:?}");

    let symbols = args.symbols.ok_or(Error::NoSymbols)?;
    let quotes = api::get_latest_quotes(&config.api_key, &symbols, API_URL_BASE).await?;
    d!("{quotes:?}");

    Ok(())
}
