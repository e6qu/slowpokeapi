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

use crate::cache::CacheResult;

/// Extension trait to convert CacheResult to DataSourceInfo
pub trait CacheResultExt<V> {
    fn to_data_source_info(&self, upstream_source: &str) -> DataSourceInfo;
}

impl<V> CacheResultExt<V> for CacheResult<V> {
    fn to_data_source_info(&self, upstream_source: &str) -> DataSourceInfo {
        DataSourceInfo {
            source: upstream_source.to_string(),
            source_timestamp_unix: self.source_timestamp.timestamp(),
            source_timestamp_utc: self.source_timestamp.to_rfc3339(),
            cached: true,
            cache_timestamp_unix: self.cached_at.map(|t| t.timestamp()),
            cache_timestamp_utc: self.cached_at.map(|t| t.to_rfc3339()),
        }
    }
}
pub use validation::ValidationError;
