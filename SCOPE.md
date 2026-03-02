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

### Fiat Currencies (~33 from ECB/Frankfurter)
ISO 4217 three-letter currency codes:
- USD, EUR, GBP, JPY, CHF, CAD, AUD, NZD, CNY, INR, etc.
- Full list: AUD, BGN, BRL, CAD, CHF, CNY, CZK, DKK, EUR, GBP, HKD, HUF, IDR, ILS, INR, ISK, JPY, KRW, MXN, MYR, NOK, NZD, PHP, PLN, RON, SEK, SGD, THB, TRY, USD, ZAR

### Cryptocurrencies (~50+ via CoinGecko/CoinCap)
- BTC (Bitcoin)
- ETH (Ethereum)
- LTC (Litecoin)
- XRP (Ripple)
- DOGE (Dogecoin)
- USDT (Tether)
- USDC (USD Coin)
- And more...

### Precious Metals (via CoinGecko)
- XAU (Gold)
- XAG (Silver)
- XPT (Platinum)
- XPD (Palladium)

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

SlowPokeAPI uses **only public, free APIs** with no paid services:

### Fiat Currency Sources

| Source | Coverage | Update Frequency | Auth | Notes |
|--------|----------|------------------|------|-------|
| [European Central Bank](https://www.ecb.europa.eu/stats/eurofxref/) | 32 currencies | Daily (around 16:00 CET) | None | Authoritative EU source |
| [Frankfurter API](https://www.frankfurter.app/) | 30+ currencies | Daily | None | Open-source, ECB-backed |
| [Open Exchange Rates](https://openexchangerates.org/) | 170+ currencies | Hourly | Free tier | 1,000 requests/month free |

### Cryptocurrency Sources

| Source | Coverage | Auth | Rate Limit | Notes |
|--------|----------|------|------------|-------|
| [CoinGecko API](https://www.coingecko.com/api/documentation) | 10,000+ coins | None (free tier) | ~10-50 req/min | Most comprehensive free crypto API |
| [CoinCap API](https://docs.coincap.io/) | 2,000+ coins | None | Generous | Simple REST API |

### Precious Metals Sources

| Source | Coverage | Auth | Notes |
|--------|----------|------|-------|
| CoinGecko API | Gold, Silver, Platinum, Palladium | None | Via crypto-style asset endpoints |

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
    │Frankfur-│     │CoinGecko│
    │ter API  │     │CoinCap  │
    │ECB XML  │     └─────────┘
    └─────────┘
```

---

## Data Source Details

### 1. European Central Bank (ECB)

**URL:** `https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml`

**Coverage:** 32 currencies (EUR as base)

**Sample Response:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<gesmes:Envelope xmlns:gesmes="http://www.gesmes.org/xml/2002-08-01">
  <Cube>
    <Cube time="2026-03-02">
      <Cube currency="USD" rate="1.0523"/>
      <Cube currency="JPY" rate="157.89"/>
      <Cube currency="GBP" rate="0.8234"/>
      ...
    </Cube>
  </Cube>
</gesmes:Envelope>
```

**Historical Data:**
- Daily rates: `https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist-90d.xml`
- Full history: `https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.xml` (since 1999)

---

### 2. Frankfurter API

**URL:** `https://api.frankfurter.app`

**Coverage:** 30+ currencies

**Endpoints:**
- Latest: `GET /latest?from={code}`
- Historical: `GET /{YYYY-MM-DD}?from={code}`
- Currencies: `GET /currencies`
- Date Range: `GET /{YYYY-MM-DD}..{YYYY-MM-DD}?from={code}`

**Sample Response:**
```json
{
  "amount": 1,
  "base": "EUR",
  "date": "2026-03-02",
  "rates": {
    "USD": 1.0523,
    "GBP": 0.8234,
    "JPY": 157.89
  }
}
```

**Advantages:**
- Open-source project
- No authentication required
- Historical data from 1999
- Well-maintained

---

### 3. CoinGecko API

**URL:** `https://api.coingecko.com/api/v3`

**Coverage:** 10,000+ cryptocurrencies including metals

**Endpoints:**
- Simple price: `GET /simple/price?ids=bitcoin,ethereum&vs_currencies=usd,eur`
- Coin list: `GET /coins/list`
- Exchange rates: `GET /exchange_rates`

**Sample Response:**
```json
{
  "bitcoin": {
    "usd": 45000.00,
    "eur": 42000.00
  },
  "ethereum": {
    "usd": 3000.00,
    "eur": 2800.00
  }
}
```

**Rate Limits:**
- Free tier: ~10-50 requests/minute
- Rate limit headers included in responses

---

### 4. CoinCap API

**URL:** `https://api.coincap.io/v2`

**Coverage:** 2,000+ cryptocurrencies

**Endpoints:**
- Rates: `GET /rates`
- Assets: `GET /assets`
- Single asset: `GET /assets/{id}`

**Sample Response:**
```json
{
  "data": {
    "id": "bitcoin",
    "rateUsd": "45000.0000000000000000"
  }
}
```

**Advantages:**
- No authentication required
- Generous rate limits
- Simple JSON responses

---

## Implementation Strategy

### Currency Rate Aggregation

Since fiat and crypto/metal rates come from different sources, SlowPokeAPI must:

1. **Fetch EUR rates from ECB/Frankfurter**
2. **Fetch crypto/metal prices from CoinGecko/CoinCap** (in USD)
3. **Calculate cross-rates** for any currency pair

### Example: Converting BTC to EUR

1. Get EUR/USD rate from Frankfurter: `1 EUR = 1.05 USD`
2. Get BTC/USD price from CoinGecko: `1 BTC = 45000 USD`
3. Calculate BTC/EUR: `45000 / 1.05 = 42857 EUR`

### Example: Converting GBP to JPY

1. Get all EUR rates from Frankfurter
2. Extract EUR/GBP and EUR/JPY rates
3. Calculate cross-rate: `JPY/GBP = (EUR/JPY) / (EUR/GBP)`

---

## Implementation Phases

### Phase 1: Core API (MVP)
- [ ] List currencies endpoint
- [ ] Latest exchange rates (fiat only via Frankfurter)
- [ ] Pair conversion (fiat only)
- [ ] Basic error handling
- [ ] In-memory caching

### Phase 2: Cryptocurrency Support
- [ ] CoinGecko API integration
- [ ] Crypto/fiat conversion
- [ ] Precious metals support
- [ ] CoinCap as fallback

### Phase 3: Historical Data
- [ ] Historical rates endpoint
- [ ] Frankfurter historical API
- [ ] ECB historical XML parsing
- [ ] SQLite storage for caching

### Phase 4: Enhanced Features
- [ ] Enriched data endpoint
- [ ] Currency metadata (symbols, names, flags)
- [ ] Rate limiting per API key

### Phase 5: Authentication & Quotas
- [ ] API key generation
- [ ] Usage tracking
- [ ] Quota management
- [ ] Plan tiers

### Phase 6: Performance & Reliability
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
| XML Parsing | quick-xml |
| Caching | moka (in-memory) / Redis (optional) |
| Database | SQLite (for historical data cache) |
| HTTP Client | reqwest |
| Async Runtime | tokio |

---

## Notes

- All currency codes use ISO 4217 standard (3 letters, uppercase)
- Exchange rates are indicative mid-market rates
- Historical fiat data available from 1999 via ECB
- All third-party APIs are **free and public** - no paid services required
- Cache responses aggressively to respect upstream rate limits
- Implement fallback chain for reliability (Frankfurter → ECB XML)
- Cross-rate calculations required for non-EUR base currencies
