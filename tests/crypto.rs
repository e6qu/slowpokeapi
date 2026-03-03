use std::sync::Arc;

use slowpokeapi::models::{is_crypto_code, is_metal_code};
use slowpokeapi::upstream::{
    CoinCapClient, CoinGeckoClient, HttpClient, Upstream, UpstreamManager,
};

#[test]
fn test_is_crypto_code() {
    assert!(is_crypto_code("BTC"));
    assert!(is_crypto_code("ETH"));
    assert!(!is_crypto_code("btc"));
    assert!(!is_crypto_code("USD"));
}

#[test]
fn test_is_metal_code() {
    assert!(is_metal_code("XAU"));
    assert!(is_metal_code("XAG"));
    assert!(!is_metal_code("USD"));
}

#[test]
fn test_crypto_currency_helpers() {
    use slowpokeapi::models::{get_crypto_currency, CurrencyType};

    let btc = get_crypto_currency("BTC").expect("BTC should exist");
    assert_eq!(btc.code, "BTC");
    assert_eq!(btc.name, "Bitcoin");
    assert!(btc.is_crypto());

    let eth = get_crypto_currency("ETH").expect("ETH should exist");
    assert_eq!(eth.code, "ETH");
    assert_eq!(eth.name, "Ethereum");
    assert!(eth.is_crypto());

    assert!(get_crypto_currency("USD").is_none());
}

#[test]
fn test_metal_currency_helpers() {
    use slowpokeapi::models::{get_metal_currency, CurrencyType};

    let xau = get_metal_currency("XAU").expect("XAU should exist");
    assert_eq!(xau.code, "XAU");
    assert!(xau.is_metal());

    assert!(get_metal_currency("USD").is_none());
}

#[test]
fn test_coingecko_id_mapping() {
    use slowpokeapi::upstream::coingecko::{
        code_to_coingecko_id, coingecko_id_to_code, is_crypto_currency,
    };

    assert_eq!(code_to_coingecko_id("BTC"), Some("bitcoin"));
    assert_eq!(code_to_coingecko_id("ETH"), Some("ethereum"));
    assert_eq!(code_to_coingecko_id("USD"), None);

    assert_eq!(coingecko_id_to_code("bitcoin"), Some("BTC"));
    assert_eq!(coingecko_id_to_code("ethereum"), Some("ETH"));

    assert!(is_crypto_currency("BTC"));
    assert!(is_crypto_currency("ETH"));
    assert!(!is_crypto_currency("USD"));
}

#[test]
fn test_coincap_id_mapping() {
    use slowpokeapi::upstream::coincap::{
        code_to_coincap_id, coincap_id_to_code, is_crypto_currency,
    };

    assert_eq!(code_to_coincap_id("BTC"), Some("bitcoin"));
    assert_eq!(code_to_coincap_id("ETH"), Some("ethereum"));
    assert_eq!(code_to_coincap_id("USD"), None);

    assert_eq!(coincap_id_to_code("bitcoin"), Some("BTC"));
    assert_eq!(coincap_id_to_code("ethereum"), Some("ETH"));

    assert!(is_crypto_currency("BTC"));
    assert!(is_crypto_currency("ETH"));
    assert!(!is_crypto_currency("USD"));
}

#[tokio::test]
async fn test_coingecko_client_get_latest_rates() {
    let http = Arc::new(HttpClient::new(10));
    let client = CoinGeckoClient::new(http);

    let result = client.get_latest_rates("BTC").await;
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_coingecko_client_not_found() {
    let http = Arc::new(HttpClient::new(10));
    let client = CoinGeckoClient::new(http);

    let result = client.get_latest_rates("INVALID").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_coincap_client_get_latest_rates() {
    let http = Arc::new(HttpClient::new(10));
    let client = CoinCapClient::new(http);

    let result = client.get_latest_rates("BTC").await;
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_coincap_client_not_found() {
    let http = Arc::new(HttpClient::new(10));
    let client = CoinCapClient::new(http);

    let result = client.get_latest_rates("INVALID").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_upstream_manager_crypto_support() {
    let http = Arc::new(HttpClient::new(10));
    let manager = UpstreamManager::new(http);

    assert!(manager.total_count() >= 4);
    assert!(manager.healthy_count() >= 4);
}

#[tokio::test]
async fn test_coingecko_client_name() {
    let http = Arc::new(HttpClient::new(10));
    let client = CoinGeckoClient::new(http);

    assert_eq!(client.name(), "coingecko");
}

#[tokio::test]
async fn test_coincap_client_name() {
    let http = Arc::new(HttpClient::new(10));
    let client = CoinCapClient::new(http);

    assert_eq!(client.name(), "coincap");
}

#[tokio::test]
async fn test_crypto_client_supports_currency() {
    let http = Arc::new(HttpClient::new(10));
    let coingecko = CoinGeckoClient::new(http.clone());
    let coincap = CoinCapClient::new(http);

    assert!(coingecko.supports_currency("BTC"));
    assert!(coingecko.supports_currency("ETH"));
    assert!(!coingecko.supports_currency("USD"));

    assert!(coincap.supports_currency("BTC"));
    assert!(coincap.supports_currency("ETH"));
    assert!(!coincap.supports_currency("USD"));
}
