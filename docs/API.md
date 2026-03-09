# API Reference

API documentation for SlowPokeAPI.

## Base URL

```
http://localhost:8080
```

## Authentication

API key authentication is optional and controlled via configuration.

### Using API Keys

Include the API key in the `X-API-Key` header:

```bash
curl -H "X-API-Key: your-api-key" http://localhost:8080/v1/latest/USD
```

### Public Endpoints

The following endpoints are accessible without authentication:

- `GET /health` - Health check
- `GET /healthz` - Liveness probe
- `GET /readyz` - Readiness probe
- `GET /livez` - Liveness probe
- `GET /metrics` - Prometheus metrics
- `GET /swagger-ui` - Swagger UI documentation
- `GET /api-docs/openapi.json` - OpenAPI specification

## Rate Limiting

Rate limiting behavior depends on configuration.

### Rate Limit Headers

| Header | Description |
|--------|-------------|
| `X-RateLimit-Limit` | Maximum requests allowed |
| `X-RateLimit-Remaining` | Remaining requests |
| `X-RateLimit-Reset` | Seconds until reset |
| `Retry-After` | Seconds to wait (on 429) |

### Quota Endpoint

Check current quota status:

```bash
GET /v1/quota
```

**Response:**

```json
{
  "api_key": "***...abcd",
  "limit": 100,
  "remaining": 95,
  "reset_seconds": 3600
}
```

## Endpoints

### GET /v1/currencies

List all supported currencies.

**Parameters:** None

**Response:**

```json
{
  "AED": "United Arab Emirates Dirham",
  "AFN": "Afghan Afghani",
  "EUR": "Euro",
  "GBP": "British Pound Sterling",
  "USD": "United States Dollar"
}
```

### GET /v1/currencies.min

List currencies in minimal format.

**Response:** Same as `/v1/currencies` but typically cached.

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
  "time_last_update_unix": 1704067200,
  "time_last_update_utc": "2024-01-01T00:00:00Z",
  "time_next_update_unix": 1704153600,
  "time_next_update_utc": "2024-01-02T00:00:00Z",
  "base_code": "USD",
  "conversion_rates": {
    "EUR": 0.9234,
    "GBP": 0.7891,
    "JPY": 148.50
  },
  "data_source": {
    "source": "frankfurter",
    "last_retrieved": "2024-01-01T00:00:00Z",
    "last_cached": null,
    "upstream_request": {
      "endpoint": "https://api.frankfurter.app/latest?from=USD"
    }
  }
}
```

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
  "time_last_update_unix": 1704067200,
  "time_last_update_utc": "2024-01-01T00:00:00Z",
  "time_next_update_unix": 1704153600,
  "time_next_update_utc": "2024-01-02T00:00:00Z",
  "base_code": "USD",
  "target_code": "EUR",
  "conversion_rate": 0.9234,
  "data_source": {
    "source": "frankfurter",
    "last_retrieved": "2024-01-01T00:00:00Z",
    "last_cached": null,
    "upstream_request": {
      "endpoint": "https://api.frankfurter.app/latest?from=USD"
    }
  }
}
```

**Response (with amount):**

```json
{
  "result": "success",
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "time_last_update_unix": 1704067200,
  "time_last_update_utc": "2024-01-01T00:00:00Z",
  "time_next_update_unix": 1704153600,
  "time_next_update_utc": "2024-01-02T00:00:00Z",
  "base_code": "USD",
  "target_code": "EUR",
  "conversion_rate": 0.9234,
  "conversion_result": 92.34,
  "data_source": {
    "source": "frankfurter",
    "last_retrieved": "2024-01-01T00:00:00Z",
    "last_cached": null,
    "upstream_request": {
      "endpoint": "https://api.frankfurter.app/latest?from=USD"
    }
  }
}
```

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
- Crypto and metal currencies are not supported

**Response:**

```json
{
  "result": "success",
  "documentation": "https://github.com/e6qu/slowpokeapi",
  "year": 2023,
  "month": 12,
  "day": 25,
  "base_code": "USD",
  "conversion_rates": {
    "EUR": 0.9123,
    "GBP": 0.7845,
    "JPY": 149.20
  },
  "data_source": {
    "source": "frankfurter",
    "last_retrieved": "2024-01-01T00:00:00Z",
    "last_cached": null,
    "upstream_request": {
      "endpoint": "https://api.frankfurter.app/2023-12-25?from=USD"
    }
  }
}
```

### GET /v1/enriched/{base_code}/{target_code}

Get exchange rate with additional metadata for the target currency.

**Parameters:**

| Name | In | Type | Required | Description |
|------|-----|------|----------|-------------|
| base_code | path | string | Yes | Base currency code |
| target_code | path | string | Yes | Target currency code |

**Response:**

```json
{
  "result": "success",
  "time_last_update_unix": 1704067200,
  "time_last_update_utc": "2024-01-01T00:00:00Z",
  "base_code": "USD",
  "target_code": "EUR",
  "conversion_rate": 0.9234,
  "target_data": {
    "code": "EUR",
    "locale": "de-DE",
    "two_letter_country_code": "DE",
    "currency_name": "Euro",
    "currency_name_short": "Euro",
    "display_symbol": "€",
    "flag_url": "https://flagcdn.com/w640/eu.png"
  },
  "data_source": {
    "source": "frankfurter",
    "last_retrieved": "2024-01-01T00:00:00Z",
    "last_cached": null,
    "upstream_request": {
      "endpoint": "https://api.frankfurter.app/latest?from=USD"
    }
  }
}
```

### GET /health

Deep health check endpoint.

**Parameters:** None

**Response (200 OK):**

```json
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "checks": {
    "database": {
      "status": "pass",
      "message": "SQLite connection healthy",
      "latency_ms": 2
    }
  }
}
```

**Response (503 Service Unavailable):**

```json
{
  "status": "unhealthy",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "checks": {
    "database": {
      "status": "fail",
      "message": "Database connection failed",
      "latency_ms": 5000
    }
  }
}
```

### GET /healthz

Liveness probe. Returns "ok" if the service is running.

**Parameters:** None

**Response (200 OK):**

```
ok
```

### GET /readyz

Readiness probe.

**Parameters:** None

**Response (200 OK):**

```
ok
```

**Response (503):**

```
not ready
```

### GET /livez

Liveness probe (alternative). Returns "ok" if the service is running.

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

Errors follow this format:

```json
{
  "result": "error",
  "error_type": "invalid-currency",
  "message": "Invalid currency code: XYZ"
}
```

### Error Types

| Error Type | HTTP Status | Description |
|------------|-------------|-------------|
| `malformed-request` | 400 | Invalid request format |
| `invalid-currency` | 400 | Invalid currency code |
| `invalid-date` | 400 | Invalid date format |
| `not-found` | 404 | Resource not found |
| `quota-reached` | 429 | Rate limit exceeded |
| `invalid-key` | 401 | Invalid API key |

## Currency Codes

### Fiat Currencies (ISO 4217)

Common codes: USD, EUR, GBP, JPY, CAD, AUD, CHF, CNY, SEK, NZD

### Cryptocurrencies

Supported: BTC, ETH, BNB, XRP, ADA, SOL, DOT, DOGE, AVAX, MATIC, LINK, UNI, LTC, BCH, XLM

### Precious Metals

| Code | Name |
|------|------|
| XAU | Gold |
| XAG | Silver |
| XPT | Platinum |
| XPD | Palladium |

## OpenAPI/Swagger

Interactive API documentation is available at:

```
http://localhost:8080/swagger-ui
```

OpenAPI JSON specification:

```
http://localhost:8080/api-docs/openapi.json
```
