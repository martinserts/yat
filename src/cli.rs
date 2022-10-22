use clap::{command, Args, Parser, Subcommand};

use crate::base::currency::Currency;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Yat (an emoji string)
    #[arg(long, short)]
    pub yat: String,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Looks up the payment addresses
    #[command(name = "lookup")]
    LookupAddress(LookupAddress),
    /// Create a new payment address
    #[command(name = "create")]
    CreateAddress(CreateAddress),
}

#[derive(Debug, Args)]
pub struct LookupAddress {
    // Currency
    #[arg(long, short, value_parser = validate_currency)]
    pub currency: Option<Currency>,
}

#[derive(Debug, Args)]
pub struct CreateAddress {
    // Currency
    #[arg(long, short, value_parser = validate_currency)]
    pub currency: Currency,
    // Wallet address
    #[arg(long, short)]
    pub address: String,
    // Name of the wallet
    #[arg(long, short)]
    pub description: Option<String>,
}

fn validate_currency(s: &str) -> Result<Currency, String> {
    Currency::try_from(s).map_err(|err| format!("{}", err))
}
