pub mod api;
pub mod currency;
pub mod error;
pub mod historical;
pub mod metadata;
pub mod rate;
pub mod validation;

pub use api::*;
pub use currency::{
    get_crypto_currency, get_metal_currency, is_crypto_code, is_metal_code, Currency, CurrencyType,
    CRYPTO_CURRENCIES, METAL_CURRENCIES,
};
pub use error::{Error, Result};
pub use historical::HistoricalRate;
pub use metadata::CurrencyMetadata;
pub use rate::{ExchangeRate, RateCollection, Source};
pub use validation::ValidationError;
