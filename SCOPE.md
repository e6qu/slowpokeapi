# SlowPokeAPI Scope Document

A clean-room reimplementation of RapidAPI's currency conversion features in Rust.

## Overview

SlowPokeAPI provides currency conversion and exchange rate services, including support for fiat currencies, cryptocurrencies, and precious metals.

---

## Core Endpoints

### 1. List Supported Currencies

Returns all available currency codes with their names.

```
GET /v1/currencies
GET /v1/currencies.min
```

**Response:**
```json
{
  "usd": "United States Dollar",
  "eur": "Euro",
  "gbp": "British Pound Sterling",
  "btc": "Bitcoin",
  "eth": "Ethereum",
  "xau": "Gold (troy ounce)",
  ...
}
```

---

### 2. Latest Exchange Rates (Standard)

Returns all exchange rates for a given base currency.

```
GET /v1/latest/{base_code}
```

**Example:** `GET /v1/latest/usd`

**Response:**
```json
{
  "result": "success",
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "time_last_update_unix": 1585267200,
  "time_last_update_utc": "Fri, 27 Mar 2020 00:00:00 +0000",
  "time_next_update_unix": 1585353700,
  "time_next_update_utc": "Sat, 28 Mar 2020 00:00:00 +0000",
  "base_code": "USD",
  "conversion_rates": {
    "USD": 1,
    "EUR": 0.9013,
    "GBP": 0.7679,
    "JPY": 119.58,
    "BTC": 0.000012,
    ...
  }
}
```

---

### 3. Pair Conversion

Convert between two specific currencies.

```
GET /v1/pair/{base_code}/{target_code}
GET /v1/pair/{base_code}/{target_code}/{amount}
```

**Example:** `GET /v1/pair/eur/gbp/100`

**Response:**
```json
{
  "result": "success",
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "time_last_update_unix": 1585267200,
  "time_last_update_utc": "Fri, 27 Mar 2020 00:00:00 +0000",
  "time_next_update_unix": 1585270800,
  "time_next_update_utc": "Sat, 28 Mar 2020 01:00:00 +0000",
  "base_code": "EUR",
  "target_code": "GBP",
  "conversion_rate": 0.8412,
  "conversion_result": 84.12
}
```

---

### 4. Historical Exchange Rates

Returns exchange rates for a specific date in the past.

```
GET /v1/history/{base_code}/{year}/{month}/{day}
GET /v1/history/{base_code}/{year}/{month}/{day}/{amount}
```

**Example:** `GET /v1/history/usd/2020/3/27`

**Response:**
```json
{
  "result": "success",
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "year": 2020,
  "month": 3,
  "day": 27,
  "base_code": "USD",
  "conversion_rates": {
    "EUR": 0.9013,
    "GBP": 0.7679,
    "JPY": 119.58,
    ...
  }
}
```

---

### 5. Enriched Currency Data

Returns exchange rate with additional metadata for the target currency.

```
GET /v1/enriched/{base_code}/{target_code}
```

**Example:** `GET /v1/enriched/gbp/jpy`

**Response:**
```json
{
  "result": "success",
  "time_last_update_unix": 1585267200,
  "time_last_update_utc": "Fri, 27 Mar 2020 00:00:00 +0000",
  "base_code": "GBP",
  "target_code": "JPY",
  "conversion_rate": 142.0543,
  "target_data": {
    "locale": "Japan",
    "two_letter_code": "JP",
    "currency_name": "Japanese Yen",
    "currency_name_short": "Yen",
    "display_symbol": "00A5",
    "flag_url": "https://example.com/flags/JP.png"
  }
}
```

---

### 6. API Request Quota

Returns the current API usage and quota information for the authenticated user.

```
GET /v1/quota
```

**Response:**
```json
{
  "result": "success",
  "quota_used": 1234,
  "quota_limit": 10000,
  "quota_remaining": 8766,
  "reset_date": "2026-04-01"
}
```

---

## Supported Currency Types

### Fiat Currencies (~165)
All ISO 4217 three-letter currency codes including:
- USD, EUR, GBP, JPY, CHF, CAD, AUD, NZD, CNY, INR, etc.
- Regional currencies: XAF, XCD, XDR, XOF, XPF
- See full list in `/data/currencies.json`

### Cryptocurrencies (~50+)
- BTC (Bitcoin)
- ETH (Ethereum)
- LTC (Litecoin)
- XRP (Ripple)
- DOGE (Dogecoin)
- USDT (Tether)
- USDC (USD Coin)
- And more...

### Precious Metals
- XAU (Gold - troy ounce)
- XAG (Silver - troy ounce)
- XPT (Platinum - troy ounce)
- XPD (Palladium - troy ounce)

---

## Error Handling

All endpoints return consistent error responses:

```json
{
  "result": "error",
  "error_type": "error-code"
}
```

### Error Types

| Error Code | Description |
|------------|-------------|
| `unsupported-code` | Currency code not supported |
| `malformed-request` | Invalid request structure |
| `invalid-key` | API key is not valid |
| `inactive-account` | Account not activated |
| `quota-reached` | API request limit exceeded |
| `no-data-available` | No historical data for requested date |
| `plan-upgrade-required` | Endpoint requires higher plan tier |

---

## Rate Limiting

### Headers
All responses include rate limit headers:
```
X-RateLimit-Limit: 10000
X-RateLimit-Remaining: 9876
X-RateLimit-Reset: 1648771200
```

### Tiers
| Plan | Requests/Month | Historical Data | Enriched Data |
|------|----------------|-----------------|---------------|
| Free | 1,500 | No | No |
| Basic | 10,000 | 1 year | No |
| Pro | 100,000 | Full history | Yes |
| Enterprise | Unlimited | Full history | Yes |

---

## Authentication

API key passed via:
- Query parameter: `?api_key=YOUR_KEY`
- Header: `Authorization: Bearer YOUR_KEY`

---

## Third-Party Data Sources

SlowPokeAPI uses **only public, free APIs** with no authentication required:

### Primary Data Sources

| Source | Type | Coverage | Update Frequency | License |
|--------|------|----------|------------------|---------|
| [fawazahmed0/exchange-api](https://github.com/fawazahmed0/exchange-api) | Fiat + Crypto + Metals | 200+ currencies | Daily | CC0-1.0 (Public Domain) |
| [European Central Bank](https://www.ecb.europa.eu/stats/eurofxref/) | Fiat only | 32 currencies | Daily | Free (ECB terms) |
| [Frankfurter API](https://api.frankfurter.app/) | Fiat only | 30+ currencies | Daily | Free |

### Crypto Data Sources (Public)

| Source | Type | Coverage | Notes |
|--------|------|----------|-------|
| [CoinGecko API](https://www.coingecko.com/api/documentation) | Crypto | 10,000+ coins | Free, no API key required (rate limited) |
| [CoinCap API](https://docs.coincap.io/) | Crypto | 2,000+ coins | Free, no API key needed |

### Historical Data Sources

| Source | Coverage | Notes |
|--------|----------|-------|
| [Frankfurter API](https://www.frankfurter.app/) | 1999-present | Free ECB-based historical rates |
| fawazahmed0/exchange-api | Limited historical | Via date-based URL paths |

### Precious Metals Data

| Source | Type | Notes |
|--------|------|-------|
| fawazahmed0/exchange-api | XAU, XAG, XPT, XPD | Included in 200+ currencies |

---

## Recommended Architecture

```
┌─────────────────┐
│   SlowPokeAPI   │
│    (Rust)       │
└────────┬────────┘
         │
    ┌────▼────┐
    │  Cache  │ (In-memory / Redis)
    │  Layer  │
    └────┬────┘
         │
    ┌────▼────────────────────────┐
    │   Data Aggregation Service  │
    └────┬────────────────┬───────┘
         │                │
    ┌────▼────┐     ┌────▼────┐
    │  Fiat   │     │  Crypto │
    │ Sources │     │ Sources │
    └────┬────┘     └────┬────┘
         │               │
    ┌────┴────┐     ┌────┴────┐
    │fawazah- │     │CoinGecko│
    │med0 API │     │CoinCap  │
    │Frankfur-│     └─────────┘
    │ter API  │
    │ECB      │
    └─────────┘
```

---

## Data Source Priority

### For Latest Rates
1. **fawazahmed0/exchange-api** (primary) - via jsDelivr CDN
   - URL: `https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1/currencies/{code}.json`
   - Covers fiat, crypto, and metals
   - No auth required, no rate limits

2. **Frankfurter API** (fallback) - for fiat only
   - URL: `https://api.frankfurter.app/latest?from={code}`
   - ECB-sourced rates
   - No auth required

3. **European Central Bank** (fallback) - for EUR base
   - URL: `https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml`
   - Authoritative source for EUR rates

### For Historical Rates
1. **Frankfurter API** (primary for fiat)
   - URL: `https://api.frankfurter.app/{YYYY-MM-DD}?from={code}`
   - Historical data from 1999

2. **fawazahmed0/exchange-api** (secondary)
   - URL: `https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@{YYYY-MM-DD}/v1/currencies/{code}.json`
   - Limited historical coverage

### For Crypto Prices
1. **CoinGecko API** (primary)
   - URL: `https://api.coingecko.com/api/v3/simple/price`
   - Free tier: ~10-30 calls/min

2. **CoinCap API** (fallback)
   - URL: `https://api.coincap.io/v2/rates`
   - No auth, generous rate limits

---

## Implementation Phases

### Phase 1: Core API (MVP)
- [ ] List currencies endpoint
- [ ] Latest exchange rates endpoint
- [ ] Pair conversion endpoint
- [ ] Basic error handling
- [ ] In-memory caching
- [ ] fawazahmed0/exchange-api integration

### Phase 2: Historical Data
- [ ] Historical rates endpoint
- [ ] Frankfurter API integration
- [ ] Data storage (SQLite)
- [ ] Date-based queries

### Phase 3: Enhanced Features
- [ ] Enriched data endpoint
- [ ] Currency metadata (symbols, names, flags)
- [ ] Rate limiting per API key

### Phase 4: Authentication & Quotas
- [ ] API key generation
- [ ] Usage tracking
- [ ] Quota management
- [ ] Plan tiers

### Phase 5: Performance & Reliability
- [ ] Redis caching (optional)
- [ ] Multiple data source fallbacks
- [ ] Rate limit headers
- [ ] Monitoring & logging

---

## Technical Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| Web Framework | Axum / Actix-web |
| Serialization | serde / serde_json |
| Caching | moka (in-memory) / Redis (optional) |
| Database | SQLite (for historical data) |
| HTTP Client | reqwest |
| Async Runtime | tokio |

---

## Notes

- All currency codes use ISO 4217 standard (3 letters, uppercase)
- Exchange rates are indicative mid-market rates
- Historical data accuracy depends on data sources
- All third-party APIs are **free and public** - no paid services
- Cache responses aggressively to minimize upstream API calls
- Implement fallback chain for reliability
