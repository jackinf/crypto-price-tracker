mod api;
mod cli;
mod config;
mod error;
mod model;
mod prelude;

use crate::config::Config;
use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let config = Config::load()?;
    let args = cli::Args::parse_args(&config);

    let symbols = args.symbols.unwrap();
    let listings = api::get_latest_listings(&config.api_key, &symbols).await?;

    for currency in listings {
        d!(
            "{} ({}) Price: ${:.2}",
            currency.name, currency.symbol, currency.quote.usd.price
        );
    }

    Ok(())
}