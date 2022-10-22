use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

use serde::Deserialize;

use crate::base::currency::{Currency, CurrencyError};

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ApiError {
    pub code: String,
    pub reason: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PaymentAddressResponse {
    pub error: Option<ApiError>,
    pub result: Option<BTreeMap<String, PaymentAddress>>,
    pub status: bool,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PaymentAddress {
    pub address: String,                    // The payment address.
    pub category: String,                   // The category of this address.
    pub default: bool,                      // Is this address the default address for the category.
    pub description: Option<String>,        // Description of the address.
    pub long_name: Option<String>, // CryptoToken long name is a defined name for the ERC20 token.
    pub settlement_network: Option<String>, // CryptoToken settlement network for the ERC20 token.
    pub short_name: Option<String>, // CryptoToken short name to identify an ERC20 token.
    pub signature: Option<String>, // Proof of ownership signature for the address.
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FriendlyPaymentAddress {
    pub currency: Currency,
    pub address: String,
    pub description: Option<String>,
}

impl Display for FriendlyPaymentAddress {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(
            fmt,
            "-------------- {} --------------",
            self.currency.get_name()
        )?;
        writeln!(fmt, "Currency code: {}", self.currency.get_hex_code())?;
        if let Some(description) = &self.description {
            writeln!(fmt, "Description: {}", description)?;
        }
        writeln!(fmt, "Address: {}", self.address)
    }
}

impl TryFrom<PaymentAddress> for FriendlyPaymentAddress {
    type Error = CurrencyError;

    fn try_from(address: PaymentAddress) -> Result<Self, Self::Error> {
        Ok(FriendlyPaymentAddress {
            currency: Currency::try_from(address.category.as_ref())?,
            address: address.address,
            description: address.description,
        })
    }
}
