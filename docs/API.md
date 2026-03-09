# API Reference

Complete API documentation for SlowPokeAPI.

## Base URL

```
http://localhost:8081
```

## Authentication

API key authentication is optional and controlled via `AUTH_ENABLED` environment variable.

### Using API Keys

Include the API key in the `X-API-Key` header:

```bash
curl -H "X-API-Key: your-api-key" http://localhost:8081/v1/latest/USD
```

### Public Endpoints

The following endpoints are always accessible without authentication:

- `GET /health` - Health check
- `GET /healthz` - Kubernetes liveness probe  
- `GET /readyz` - Kubernetes readiness probe
- `GET /livez` - Kubernetes liveness probe
- `GET /metrics` - Prometheus metrics
- `GET /swagger-ui` - Swagger UI documentation
- `GET /api-docs/openapi.json` - OpenAPI specification

## Rate Limiting

Rate limiting is enabled by default with three tiers:

| Tier | Rate | Burst |
|------|------|-------|
| Global | 250 req/s | 500 |
| Authenticated | 25 req/s | 50 |
| Anonymous | 5 req/s | 10 |

### Rate Limit Headers

| Header | Description |
|--------|-------------|
| `X-RateLimit-Limit` | Maximum requests allowed |
| `X-RateLimit-Remaining` | Remaining requests |
| `X-RateLimit-Reset` | Seconds until reset |
| `Retry-After` | Seconds to wait (on 429) |

### Quota Endpoint

Check your current quota status:

```bash
GET /v1/quota
```

**Response:**

```json
{
  "plan_quota": 100,
  "requests_remaining": 95,
  "refresh_day_of_month": 1
}
```

## Endpoints

### GET /v1/currencies

List all supported currencies.

**Parameters:** None

**Response:**

```json
{
  "result": "success",
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "terms_of_use": "https://github.com/e6qu/slowpokeapi/blob/main/LICENSE",
  "supported_codes": [
    ["USD", "United States Dollar"],
    ["EUR", "Euro"],
    ["GBP", "British Pound Sterling"],
    ["BTC", "Bitcoin"],
    ["ETH", "Ethereum"],
    ["XAU", "Gold"]
  ]
}
```

### GET /v1/latest/{base_code}

Get latest exchange rates for a base currency.

**Parameters:**

| Name | In | Type | Required | Description |
|------|-----|------|----------|-------------|
| base_code | path | string | Yes | 3-letter currency code (e.g., USD, EUR, BTC) |

**Response:**

```json
{
  "result": "success",
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "terms_of_use": "https://github.com/e6qu/slowpokeapi/blob/main/LICENSE",
  "time_last_update_unix": 1704067200,
  "time_last_update_utc": "Mon, 01 Jan 2024 00:00:00 +0000",
  "time_next_update_unix": 1704153600,
  "time_next_update_utc": "Tue, 02 Jan 2024 00:00:00 +0000",
  "base_code": "USD",
  "conversion_rates": {
    "EUR": 0.9234,
    "GBP": 0.7891,
    "JPY": 148.50,
    "BTC": 0.00002341
  }
}
```

**Errors:**

| Status | Code | Description |
|--------|------|-------------|
| 400 | INVALID_CURRENCY_CODE | Currency code must be 3 letters |
| 404 | CURRENCY_NOT_FOUND | Unknown currency code |
| 429 | RATE_LIMIT_EXCEEDED | Rate limit exceeded |

### GET /v1/pair/{base_code}/{target_code}

Convert between two currencies.

**Parameters:**

| Name | In | Type | Required | Description |
|------|-----|------|----------|-------------|
| base_code | path | string | Yes | Source currency code |
| target_code | path | string | Yes | Target currency code |
| amount | query | number | No | Amount to convert |

**Response (without amount):**

```json
{
  "result": "success",
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "terms_of_use": "https://github.com/e6qu/slowpokeapi/blob/main/LICENSE",
  "time_last_update_unix": 1704067200,
  "time_last_update_utc": "Mon, 01 Jan 2024 00:00:00 +0000",
  "time_next_update_unix": 1704153600,
  "time_next_update_utc": "Tue, 02 Jan 2024 00:00:00 +0000",
  "base_code": "USD",
  "target_code": "EUR",
  "conversion_rate": 0.9234
}
```

**Response (with amount):**

```json
{
  "result": "success",
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "terms_of_use": "https://github.com/e6qu/slowpokeapi/blob/main/LICENSE",
  "time_last_update_unix": 1704067200,
  "time_last_update_utc": "Mon, 01 Jan 2024 00:00:00 +0000",
  "time_next_update_unix": 1704153600,
  "time_next_update_utc": "Tue, 02 Jan 2024 00:00:00 +0000",
  "base_code": "USD",
  "target_code": "EUR",
  "conversion_rate": 0.9234,
  "conversion_result": 92.34
}
```

**Errors:**

| Status | Code | Description |
|--------|------|-------------|
| 400 | INVALID_CURRENCY_CODE | Invalid currency code format |
| 400 | SAME_CURRENCY | Base and target are identical |
| 400 | INVALID_AMOUNT | Amount must be positive and finite |
| 404 | CURRENCY_NOT_FOUND | Unknown currency code |
| 404 | RATE_NOT_AVAILABLE | Exchange rate not available |
| 429 | RATE_LIMIT_EXCEEDED | Rate limit exceeded |

### GET /v1/history/{base_code}/{year}/{month}/{day}

Get historical exchange rates for a specific date.

**Parameters:**

| Name | In | Type | Required | Description |
|------|-----|------|----------|-------------|
| base_code | path | string | Yes | Base currency code |
| year | path | integer | Yes | 4-digit year |
| month | path | integer | Yes | Month (1-12) |
| day | path | integer | Yes | Day (1-31) |

**Notes:**
- Minimum date: 1999-01-04 (Frankfurter API limitation)
- Future dates are rejected

**Response:**

```json
{
  "result": "success",
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "terms_of_use": "https://github.com/e6qu/slowpokeapi/blob/main/LICENSE",
  "year": 2023,
  "month": 12,
  "day": 25,
  "base_code": "USD",
  "conversion_rates": {
    "EUR": 0.9123,
    "GBP": 0.7845,
    "JPY": 149.20
  }
}
```

**Errors:**

| Status | Code | Description |
|--------|------|-------------|
| 400 | INVALID_DATE | Invalid date format |
| 400 | FUTURE_DATE | Date is in the future |
| 400 | DATE_TOO_OLD | Date before 1999-01-04 |
| 404 | CURRENCY_NOT_FOUND | Unknown currency code |
| 404 | DATA_NOT_AVAILABLE | No data for this date |
| 429 | RATE_LIMIT_EXCEEDED | Rate limit exceeded |

### GET /v1/enriched/{base_code}

Get exchange rates with additional metadata.

**Parameters:**

| Name | In | Type | Required | Description |
|------|-----|------|----------|-------------|
| base_code | path | string | Yes | Base currency code |

**Response:**

```json
{
  "result": "success",
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "terms_of_use": "https://github.com/e6qu/slowpokeapi/blob/main/LICENSE",
  "time_last_update_unix": 1704067200,
  "time_last_update_utc": "Mon, 01 Jan 2024 00:00:00 +0000",
  "time_next_update_unix": 1704153600,
  "time_next_update_utc": "Tue, 02 Jan 2024 00:00:00 +0000",
  "base_code": "USD",
  "conversion_rates": {
    "EUR": {
      "rate": 0.9234,
      "rate_for_amount": 9.23,
      "currency_name": "Euro"
    },
    "GBP": {
      "rate": 0.7891,
      "rate_for_amount": 7.89,
      "currency_name": "British Pound Sterling"
    }
  }
}
```

### GET /health

Combined health check endpoint.

**Parameters:** None

**Response (200 OK):**

```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z",
  "version": "1.0.0",
  "database": "connected",
  "sync_enabled": false,
  "sync_peers": 0
}
```

**Response (503 Service Unavailable):**

```json
{
  "status": "unhealthy",
  "timestamp": "2024-01-01T00:00:00Z",
  "version": "1.0.0",
  "database": "disconnected",
  "error": "Database connection failed"
}
```

### GET /healthz

Kubernetes liveness probe.

**Parameters:** None

**Response (200 OK):**

```json
{
  "status": "ok"
}
```

### GET /readyz

Kubernetes readiness probe.

**Parameters:** None

**Response (200 OK):**

```json
{
  "status": "ready",
  "database": "connected"
}
```

**Response (503):**

```json
{
  "status": "not_ready",
  "database": "disconnected"
}
```

### GET /livez

Kubernetes liveness probe (alternative).

**Parameters:** None

**Response:** Same as `/healthz`

### GET /metrics

Prometheus metrics endpoint.

**Parameters:** None

**Response:** Plain text in Prometheus exposition format

```
# HELP slowpokeapi_requests_total Total number of HTTP requests
# TYPE slowpokeapi_requests_total counter
slowpokeapi_requests_total{method="GET",status="200",path="/v1/latest/USD"} 42

# HELP slowpokeapi_request_duration_seconds HTTP request duration
# TYPE slowpokeapi_request_duration_seconds histogram
slowpokeapi_request_duration_seconds_bucket{le="0.1"} 35
...
```

## Error Responses

All errors follow this format:

```json
{
  "result": "error",
  "error-type": "ERROR_TYPE",
  "error-message": "Human-readable description"
}
```

### Error Types

| Error Type | HTTP Status | Description |
|------------|-------------|-------------|
| `malformed-request` | 400 | Invalid request format |
| `invalid-currency-code` | 400 | Currency code must be 3 letters |
| `same-currency` | 400 | Base and target currencies are identical |
| `invalid-amount` | 400 | Amount must be positive |
| `invalid-date` | 400 | Invalid date format |
| `future-date` | 400 | Date is in the future |
| `date-too-old` | 400 | Date before minimum (1999-01-04) |
| `unsupported-code` | 404 | Unknown currency code |
| `currency-not-found` | 404 | Currency not found |
| `rate-not-available` | 404 | Exchange rate not available |
| `data-not-available` | 404 | Historical data not available |
| `metal-rates-unsupported` | 400 | Metal currencies not supported for this operation |
| `rate-limit-exceeded` | 429 | Too many requests |
| `invalid-api-key` | 401 | Invalid or missing API key |
| `internal-error` | 500 | Server error |

## Currency Codes

### Fiat Currencies (ISO 4217)

Common codes: USD, EUR, GBP, JPY, CAD, AUD, CHF, CNY, SEK, NZD

### Cryptocurrencies

| Code | Name |
|------|------|
| BTC | Bitcoin |
| ETH | Ethereum |
| BNB | Binance Coin |
| XRP | Ripple |
| ADA | Cardano |
| SOL | Solana |
| DOT | Polkadot |
| DOGE | Dogecoin |
| AVAX | Avalanche |
| MATIC | Polygon |
| LINK | Chainlink |
| UNI | Uniswap |
| LTC | Litecoin |
| BCH | Bitcoin Cash |
| XLM | Stellar |

### Precious Metals

| Code | Name |
|------|------|
| XAU | Gold |
| XAG | Silver |
| XPT | Platinum |
| XPD | Palladium |

## RapidAPI Compatibility

This API is designed to be compatible with RapidAPI's exchange rate API specification where possible.

### Differences from RapidAPI

1. **Authentication**: Uses `X-API-Key` header instead of `X-RapidAPI-Key`
2. **Base URL**: Different host (your deployment vs RapidAPI)
3. **Rate Limits**: Configurable per deployment
4. **Additional Features**:
   - Cryptocurrency support
   - Precious metals support
   - CRDT-based distributed sync
   - Self-hosted option

### Migration from RapidAPI

Replace the base URL and header:

```bash
# RapidAPI
curl -H "X-RapidAPI-Key: your-key" \
  https://v6.exchangerate-api.com/v6/latest/USD

# SlowPokeAPI
curl -H "X-API-Key: your-key" \
  http://your-host:8081/v1/latest/USD
```

## SDK Examples

### JavaScript/TypeScript

```typescript
const API_BASE = 'http://localhost:8081';
const API_KEY = 'your-api-key';

async function getLatestRates(baseCode: string) {
  const response = await fetch(`${API_BASE}/v1/latest/${baseCode}`, {
    headers: API_KEY ? { 'X-API-Key': API_KEY } : {}
  });
  return response.json();
}
```

### Python

```python
import requests

API_BASE = 'http://localhost:8081'
API_KEY = 'your-api-key'

def get_latest_rates(base_code):
    headers = {'X-API-Key': API_KEY} if API_KEY else {}
    response = requests.get(f'{API_BASE}/v1/latest/{base_code}', headers=headers)
    return response.json()
```

### Go

```go
package main

import (
    "encoding/json"
    "net/http"
)

const apiBase = "http://localhost:8081"

func getLatestRates(baseCode string) (map[string]interface{}, error) {
    resp, err := http.Get(apiBase + "/v1/latest/" + baseCode)
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()
    
    var result map[string]interface{}
    json.NewDecoder(resp.Body).Decode(&result)
    return result, nil
}
```

### Rust

```rust
use serde_json::Value;

const API_BASE: &str = "http://localhost:8081";

async fn get_latest_rates(base_code: &str) -> Result<Value, reqwest::Error> {
    let url = format!("{}/v1/latest/{}", API_BASE, base_code);
    let response = reqwest::get(&url).await?;
    response.json().await
}
```

## OpenAPI/Swagger

Interactive API documentation is available at:

```
http://localhost:8081/swagger-ui
```

OpenAPI JSON specification:

```
http://localhost:8081/api-docs/openapi.json
```
