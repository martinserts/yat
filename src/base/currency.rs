use std::collections::HashMap;

static CODE_PREFIX: &str = "0x";

const CURRENCIES: [(u16, &str); 57] = [
    (0x1001, "Monero standard address"),
    (0x1002, "Monero sub address"),
    (0x1003, "Bitcoin address"),
    (0x1004, "Ethereum address"),
    (0x1010, "Binance Coin"),
    (0x1012, "Cardano"),
    (0x1013, "Dogecoin"),
    (0x1014, "XRP"),
    (0x1015, "Polkadot"),
    (0x1016, "Internet Computer"),
    (0x1017, "Bitcoin Cash"),
    (0x1019, "Litecoin"),
    (0x101C, "Stellar"),
    (0x101D, "Solana"),
    (0x101E, "VeChain"),
    (0x101F, "Ethereum Classic"),
    (0x1020, "EOS"),
    (0x1021, "THETA"),
    (0x1023, "TRON"),
    (0x1024, "Filecoin"),
    (0x1027, "Neo"),
    (0x1029, "Polygon"),
    (0x102B, "Terra"),
    (0x102C, "Bitcoin SV"),
    (0x102F, "IOTA"),
    (0x1030, "Tezos"),
    (0x1031, "Klaytn"),
    (0x1032, "Cosmos"),
    (0x1035, "Avalanche"),
    (0x1036, "Kusama"),
    (0x1038, "Algorand"),
    (0x103C, "Dash"),
    (0x103D, "Zcash"),
    (0x103F, "Waves"),
    (0x1040, "Elrond"),
    (0x1042, "NEM"),
    (0x1043, "Decred"),
    (0x1049, "TerraUSD"),
    (0x104B, "Zilliqa"),
    (0x104D, "Nano"),
    (0x1050, "Qtum"),
    (0x1052, "NEAR Protocol"),
    (0x1054, "Stacks"),
    (0x1056, "Ontology"),
    (0x1059, "Bitcoin Gold"),
    (0x105A, "Theta Fuel"),
    (0x105C, "DigiByte"),
    (0x105D, "Fantom"),
    (0x105E, "Horizen"),
    (0x1060, "Siacoin"),
    (0x1063, "OMG Network"),
    (0x1064, "Helium"),
    (0x1065, "ICON"),
    (0x1067, "Ravencoin"),
    (0x106C, "Harmony"),
    (0x106D, "Celo"),
    (0x106F, "Arweave"),
];

lazy_static! {
    static ref CURRENCY_BY_ID: HashMap<u16, Currency> =
        HashMap::from_iter(CURRENCIES.into_iter().map(|ccy @ (k, _)| (k, ccy.into())));
}

lazy_static! {
    static ref CURRENCY_BY_NAME: HashMap<String, Currency> = HashMap::from_iter(
        CURRENCIES
            .into_iter()
            .map(|ccy @ (_, v)| (v.to_lowercase(), ccy.into()))
    );
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum CurrencyError {
    #[error("Currency code `{0}` was not found")]
    MissingCurrencyCode(u16),
    #[error("Currency `{0}` was not found")]
    MissingCurrency(String),
    #[error("Invalid currency code `{0}`")]
    InvalidCurrencyCode(#[from] std::num::ParseIntError),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Currency {
    code: u16,
    name: &'static str,
}

impl Currency {
    pub fn get_hex_code(&self) -> String {
        format!("{:#x}", self.code)
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }
}

impl From<(u16, &'static str)> for Currency {
    fn from(ccy: (u16, &'static str)) -> Self {
        Self {
            code: ccy.0,
            name: ccy.1,
        }
    }
}

impl TryFrom<u16> for Currency {
    type Error = CurrencyError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        match CURRENCY_BY_ID.get(&code) {
            Some(ccy) => Ok(ccy.clone()),
            None => Err(CurrencyError::MissingCurrencyCode(code)),
        }
    }
}

impl TryFrom<&str> for Currency {
    type Error = CurrencyError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        if name.starts_with(CODE_PREFIX) {
            let code = u16::from_str_radix(name.trim_start_matches(CODE_PREFIX), 16)?;
            return Currency::try_from(code);
        }

        match CURRENCY_BY_NAME.get(&name.to_lowercase()) {
            Some(ccy) => Ok(ccy.clone()),
            None => Err(CurrencyError::MissingCurrency(name.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_get_currency_name() {
        let ccy = Currency {
            code: 1,
            name: "NAME",
        };
        assert_eq!(ccy.get_name(), "NAME");
    }

    #[test]
    fn it_can_get_currency_code() {
        let ccy = Currency {
            code: 0x1012,
            name: "NAME",
        };
        assert_eq!(ccy.get_hex_code(), "0x1012");
    }

    #[test]
    fn it_creates_currency_from_tuple() {
        assert_eq!(
            Currency::from((0x1012, "Cardano")),
            Currency {
                code: 0x1012,
                name: "Cardano",
            }
        );
    }

    mod try_from_u16 {
        use super::*;

        #[test]
        fn it_finds_currency_by_code() {
            assert_eq!(
                Currency::try_from(0x1012),
                Ok(Currency {
                    code: 0x1012,
                    name: "Cardano",
                })
            );
        }

        #[test]
        fn it_fails_if_currency_code_is_not_found() {
            assert_eq!(
                Currency::try_from(0xffff),
                Err(CurrencyError::MissingCurrencyCode(0xffff))
            );
        }
    }

    mod try_from_str {
        use super::*;

        #[test]
        fn it_finds_currency_by_code() {
            assert_eq!(
                Currency::try_from("0x1012"),
                Ok(Currency {
                    code: 0x1012,
                    name: "Cardano",
                })
            );
        }

        #[test]
        fn it_finds_currency_by_name() {
            assert_eq!(
                Currency::try_from("Cardano"),
                Ok(Currency {
                    code: 0x1012,
                    name: "Cardano",
                })
            );
        }

        #[test]
        fn it_fails_if_currency_is_not_found() {
            assert_eq!(
                Currency::try_from("UNKNOWN"),
                Err(CurrencyError::MissingCurrency("UNKNOWN".to_owned()))
            );
        }
    }
}
