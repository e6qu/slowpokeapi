use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CurrencyType {
    Fiat,
    Crypto,
    Metal,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Currency {
    pub code: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    pub currency_type: CurrencyType,
}

impl Currency {
    pub fn is_fiat(&self) -> bool {
        matches!(self.currency_type, CurrencyType::Fiat)
    }

    pub fn is_crypto(&self) -> bool {
        matches!(self.currency_type, CurrencyType::Crypto)
    }

    pub fn is_metal(&self) -> bool {
        matches!(self.currency_type, CurrencyType::Metal)
    }
}

pub const CRYPTO_CURRENCIES: &[(&str, &str, &str)] = &[
    ("BTC", "Bitcoin", "₿"),
    ("ETH", "Ethereum", "Ξ"),
    ("XRP", "Ripple", "XRP"),
    ("LTC", "Litecoin", "Ł"),
    ("BCH", "Bitcoin Cash", "BCH"),
    ("ADA", "Cardano", "ADA"),
    ("DOT", "Polkadot", "DOT"),
    ("LINK", "Chainlink", "LINK"),
    ("XLM", "Stellar", "XLM"),
    ("DOGE", "Dogecoin", "Ð"),
    ("SOL", "Solana", "SOL"),
    ("MATIC", "Polygon", "MATIC"),
    ("AVAX", "Avalanche", "AVAX"),
    ("UNI", "Uniswap", "UNI"),
    ("ATOM", "Cosmos", "ATOM"),
];

pub const METAL_CURRENCIES: &[(&str, &str, &str)] = &[
    ("XAU", "Gold (Troy Ounce)", "Au"),
    ("XAG", "Silver (Troy Ounce)", "Ag"),
    ("XPT", "Platinum (Troy Ounce)", "Pt"),
    ("XPD", "Palladium (Troy Ounce)", "Pd"),
];

pub fn is_crypto_code(code: &str) -> bool {
    CRYPTO_CURRENCIES.iter().any(|(c, _, _)| *c == code)
}

pub fn is_metal_code(code: &str) -> bool {
    METAL_CURRENCIES.iter().any(|(c, _, _)| *c == code)
}

pub fn get_crypto_currency(code: &str) -> Option<Currency> {
    CRYPTO_CURRENCIES
        .iter()
        .find(|(c, _, _)| *c == code)
        .map(|(code, name, symbol)| Currency {
            code: code.to_string(),
            name: name.to_string(),
            symbol: Some(symbol.to_string()),
            currency_type: CurrencyType::Crypto,
        })
}

pub fn get_metal_currency(code: &str) -> Option<Currency> {
    METAL_CURRENCIES
        .iter()
        .find(|(c, _, _)| *c == code)
        .map(|(code, name, symbol)| Currency {
            code: code.to_string(),
            name: name.to_string(),
            symbol: Some(symbol.to_string()),
            currency_type: CurrencyType::Metal,
        })
}
