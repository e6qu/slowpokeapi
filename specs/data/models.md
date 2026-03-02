# Data Models

## Core Domain Models

### Currency

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub code: String,           // ISO 4217 code (e.g., "USD")
    pub name: String,           // Full name (e.g., "United States Dollar")
    pub symbol: Option<String>, // Display symbol (e.g., "$")
    pub currency_type: CurrencyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurrencyType {
    Fiat,
    Crypto,
    Metal,
}

impl Currency {
    pub fn is_fiat(&self) -> bool {
        matches!(self.currency_type, CurrencyType::Fiat)
    }
}
```

### Exchange Rate

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub base_code: String,
    pub target_code: String,
    pub rate: f64,
    pub timestamp: DateTime<Utc>,
    pub source: Source,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    Frankfurter,
    FawazAhmed,
    CoinGecko,
    CoinCap,
    Cached,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateCollection {
    pub base_code: String,
    pub rates: HashMap<String, f64>,
    pub timestamp: DateTime<Utc>,
    pub source: Source,
}
```

### Historical Rate

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalRate {
    pub base_code: String,
    pub date: NaiveDate,
    pub rates: HashMap<String, f64>,
    pub source: Source,
}
```

### Currency Metadata (Enriched)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyMetadata {
    pub code: String,
    pub locale: String,
    pub two_letter_country_code: String,
    pub currency_name: String,
    pub currency_name_short: String,
    pub display_symbol: String,
    pub flag_url: String,
}
```

## API Request/Response Models

### Requests

```rust
// All requests are GET with path parameters, no body models needed
// Query parameters for optional features:

#[derive(Debug, Deserialize)]
pub struct LatestQuery {
    pub base_code: String,
}

#[derive(Debug, Deserialize)]
pub struct PairQuery {
    pub base_code: String,
    pub target_code: String,
    pub amount: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    pub base_code: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub amount: Option<f64>,
}
```

### Responses

```rust
#[derive(Debug, Serialize)]
pub struct LatestRatesResponse {
    pub result: ResponseResult,
    pub documentation: String,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: i64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub conversion_rates: HashMap<String, f64>,
}

#[derive(Debug, Serialize)]
pub struct PairResponse {
    pub result: ResponseResult,
    pub documentation: String,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: i64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub target_code: String,
    pub conversion_rate: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_result: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct HistoricalResponse {
    pub result: ResponseResult,
    pub documentation: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub base_code: String,
    pub conversion_rates: HashMap<String, f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_results: Option<HashMap<String, f64>>,
}

#[derive(Debug, Serialize)]
pub struct EnrichedResponse {
    pub result: ResponseResult,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub base_code: String,
    pub target_code: String,
    pub conversion_rate: f64,
    pub target_data: CurrencyMetadata,
}

#[derive(Debug, Serialize)]
pub struct QuotaResponse {
    pub result: ResponseResult,
    pub quota_used: u64,
    pub quota_limit: u64,
    pub quota_remaining: u64,
    pub reset_date: String,
}

#[derive(Debug, Serialize)]
pub struct CurrenciesResponse {
    #[serde(flatten)]
    pub currencies: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub result: ResponseResult,
    pub error_type: ErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum ResponseResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Serialize)]
pub enum ErrorType {
    #[serde(rename = "unsupported-code")]
    UnsupportedCode,
    #[serde(rename = "malformed-request")]
    MalformedRequest,
    #[serde(rename = "invalid-key")]
    InvalidKey,
    #[serde(rename = "inactive-account")]
    InactiveAccount,
    #[serde(rename = "quota-reached")]
    QuotaReached,
    #[serde(rename = "no-data-available")]
    NoDataAvailable,
    #[serde(rename = "plan-upgrade-required")]
    PlanUpgradeRequired,
    #[serde(rename = "internal-error")]
    InternalError,
}
```

## Internal Models

### Cache Entry

```rust
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub data: T,
    pub cached_at: DateTime<Utc>,
    pub ttl: Duration,
    pub source: Source,
}

impl<T> CacheEntry<T> {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.cached_at + self.ttl
    }
    
    pub fn is_stale(&self) -> bool {
        // Stale but not expired - can be served while refreshing
        let age = Utc::now() - self.cached_at;
        age > self.ttl * 2
    }
}
```

### Sync State

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    pub rates: HashMap<String, RateEntry>,
    pub version: u64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateEntry {
    pub base_code: String,
    pub target_code: String,
    pub rate: f64,
    pub timestamp: i64,
    pub source: String,
}

impl RateEntry {
    pub fn key(&self) -> String {
        format!("{}_{}", self.base_code, self.target_code)
    }
}
```

### Upstream Response Models

```rust
// Frankfurter API response
#[derive(Debug, Deserialize)]
pub struct FrankfurterResponse {
    pub amount: f64,
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
}

// fawazahmed0 API response
#[derive(Debug, Deserialize)]
pub struct FawazResponse {
    pub date: String,
    #[alias = "eur"]
    pub rates: HashMap<String, f64>,
}

// CoinGecko price response
#[derive(Debug, Deserialize)]
pub struct CoinGeckoPriceResponse {
    #[serde(flatten)]
    pub prices: HashMap<String, HashMap<String, f64>>,
}

// CoinCap rates response
#[derive(Debug, Deserialize)]
pub struct CoinCapResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct CoinCapRate {
    pub id: String,
    pub symbol: String,
    pub rate_usd: String, // Comes as string
}
```

## Validation

```rust
impl Currency {
    pub fn validate_code(code: &str) -> Result<(), Error> {
        if code.len() != 3 || !code.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(Error::validation("Invalid currency code format"));
        }
        Ok(())
    }
}

impl ExchangeRate {
    pub fn validate(&self) -> Result<(), Error> {
        Currency::validate_code(&self.base_code)?;
        Currency::validate_code(&self.target_code)?;
        if self.rate <= 0.0 {
            return Err(Error::validation("Rate must be positive"));
        }
        Ok(())
    }
}
```

## Conversion Utilities

```rust
impl RateCollection {
    pub fn convert(&self, target: &str, amount: f64) -> Option<f64> {
        self.rates.get(target).map(|rate| amount * rate)
    }
    
    pub fn cross_rate(&self, from: &str, to: &str) -> Option<f64> {
        // If base is EUR and we want USD -> GBP
        // rate = USD_rate / GBP_rate
        match (self.rates.get(from), self.rates.get(to)) {
            (Some(from_rate), Some(to_rate)) => Some(to_rate / from_rate),
            _ => None,
        }
    }
}

impl HistoricalRate {
    pub fn to_response(&self, amount: Option<f64>) -> HistoricalResponse {
        let conversion_results = amount.map(|a| {
            self.rates.iter()
                .map(|(k, v)| (k.clone(), a * v))
                .collect()
        });
        
        HistoricalResponse {
            result: ResponseResult::Success,
            documentation: "https://github.com/e6qu/slowpokeapi".to_string(),
            year: self.date.year(),
            month: self.date.month(),
            day: self.date.day(),
            base_code: self.base_code.clone(),
            conversion_rates: self.rates.clone(),
            conversion_results,
        }
    }
}
```
