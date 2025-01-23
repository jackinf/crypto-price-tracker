use crate::config::Config;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Comma-separated list of cryptocurrency symbols
    #[arg(short, long)]
    pub symbols: Option<String>,
}

impl Args {
    pub fn parse_args(config: &Config) -> Self {
        let mut args = Args::parse();
        if args.symbols.is_none() {
            args.symbols = Some(config.default_symbols.join(","));
        }
        args
    }
}