use api::{error::ClientError, payment_address::FriendlyPaymentAddress};
use clap::{error::ErrorKind, CommandFactory, Parser};
use cli::{Cli, LookupAddress};
use settings::Settings;

use crate::{
    base::yat::Yat,
    cli::Commands,
    settings::{loader::Loader, settings_reader::EnvSettingsReader},
};

#[macro_use]
extern crate lazy_static;

mod api;
mod base;
mod cli;
mod settings;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let settings = Loader::new(EnvSettingsReader).load().unwrap_or_else(|err| {
        log::error!("{}", err);
        std::process::exit(1);
    });

    let args = Cli::parse();
    let yat = Yat::new(&settings, &args.yat).unwrap_or_else(|err| {
        let mut cmd = Cli::command();
        cmd.error(ErrorKind::InvalidValue, err).exit();
    });

    if let Err(err) = execute(settings, args, yat).await {
        log::error!("{}", err);
    }
}

fn should_display_address(address: &FriendlyPaymentAddress, lookup: &LookupAddress) -> bool {
    if let Some(currency) = &lookup.currency {
        return address.currency == *currency;
    }
    true
}

async fn execute(settings: Settings, args: Cli, yat: Yat) -> Result<(), ClientError> {
    let client = api::client::ApiClient::new(&settings);
    match args.command {
        Commands::LookupAddress(lookup) => {
            let addresses: Vec<FriendlyPaymentAddress> = client
                .fetch_payment_addresses(&yat)
                .await?
                .into_iter()
                .filter(|a| should_display_address(a, &lookup))
                .collect();
            if addresses.is_empty() {
                println!("No payment addresses found!");
            } else {
                for address in addresses {
                    println!("{}", address);
                }
            }
        }
        Commands::CreateAddress(create) => {
            client.create_payment_address(&yat, create).await?;
            println!("Address successfully created!");
        }
    }
    Ok(())
}
