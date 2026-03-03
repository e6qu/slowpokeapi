# Task: Cryptocurrency Support

## Status
[x] Done

## Description
Add CoinGecko and CoinCap upstream clients for crypto/metal rates.

## Files
- `src/upstream/coingecko.rs` - CoinGecko client
- `src/upstream/coincap.rs` - CoinCap client
- `src/upstream/manager.rs` - Updated for crypto routing
- `src/models/currency.rs` - Crypto/metal currency definitions
- `src/handlers/latest.rs` - Updated validation
- `tests/crypto.rs` - Crypto tests

## Notes
- Supports BTC, ETH, and 13 other cryptocurrencies
- Supports XAU, XAG, XPT, XPD metals
- Automatic routing based on currency code
- Historical rates supported for crypto
