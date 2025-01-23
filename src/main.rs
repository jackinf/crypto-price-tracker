mod api;
mod cli;
mod config;
mod error;
mod prelude;

use crate::config::Config;
use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let config = Config::load()?;
    let args = cli::Args::parse_args(&config);

    let listings = api::get_latest_listings(&config.api_key).await?;
    d!("{listings:?}");

    let symbols = args.symbols.ok_or(Error::NoSymbols)?;
    let quotes = api::get_latest_quotes(&config.api_key, &symbols).await?;
    d!("{quotes:?}");

    Ok(())
}
