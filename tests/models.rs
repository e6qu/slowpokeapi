use axum::response::IntoResponse;
use slowpokeapi::models::*;

#[test]
fn currency_serialization_roundtrip() {
    let currency = Currency {
        code: "USD".to_string(),
        name: "US Dollar".to_string(),
        symbol: Some("$".to_string()),
        currency_type: CurrencyType::Fiat,
    };

    let json = serde_json::to_string(&currency).unwrap();
    let deserialized: Currency = serde_json::from_str(&json).unwrap();

    assert_eq!(currency.code, deserialized.code);
    assert_eq!(currency.name, deserialized.name);
    assert_eq!(currency.symbol, deserialized.symbol);
}

#[test]
fn currency_type_serialization() {
    let fiat = CurrencyType::Fiat;
    let crypto = CurrencyType::Crypto;
    let metal = CurrencyType::Metal;

    assert_eq!(
        "fiat",
        serde_json::to_string(&fiat).unwrap().trim_matches('"')
    );
    assert_eq!(
        "crypto",
        serde_json::to_string(&crypto).unwrap().trim_matches('"')
    );
    assert_eq!(
        "metal",
        serde_json::to_string(&metal).unwrap().trim_matches('"')
    );
}

#[test]
fn currency_is_type_checks() {
    let fiat = Currency {
        code: "USD".to_string(),
        name: "US Dollar".to_string(),
        symbol: Some("$".to_string()),
        currency_type: CurrencyType::Fiat,
    };
    let crypto = Currency {
        code: "BTC".to_string(),
        name: "Bitcoin".to_string(),
        symbol: Some("₿".to_string()),
        currency_type: CurrencyType::Crypto,
    };

    assert!(fiat.is_fiat());
    assert!(!fiat.is_crypto());
    assert!(crypto.is_crypto());
    assert!(!crypto.is_fiat());
}

#[test]
fn exchange_rate_serialization() {
    let rate = ExchangeRate {
        base_code: "USD".to_string(),
        target_code: "EUR".to_string(),
        rate: 0.92,
        timestamp: chrono::Utc::now(),
        source: Source::Frankfurter,
    };

    let json = serde_json::to_string(&rate).unwrap();
    let deserialized: ExchangeRate = serde_json::from_str(&json).unwrap();

    assert_eq!(rate.base_code, deserialized.base_code);
    assert_eq!(rate.target_code, deserialized.target_code);
    assert!((rate.rate - deserialized.rate).abs() < f64::EPSILON);
}

#[test]
fn source_serialization() {
    let sources = vec![
        (Source::Frankfurter, "Frankfurter"),
        (Source::FawazAhmed, "FawazAhmed"),
        (Source::CoinGecko, "CoinGecko"),
        (Source::CoinCap, "CoinCap"),
        (Source::Cached, "Cached"),
    ];

    for (source, expected) in sources {
        let json = serde_json::to_string(&source).unwrap();
        assert!(json.contains(expected));
    }
}

#[test]
fn rate_collection_serialization() {
    let mut rates = std::collections::HashMap::new();
    rates.insert("EUR".to_string(), 0.92);
    rates.insert("GBP".to_string(), 0.79);

    let collection = RateCollection {
        base_code: "USD".to_string(),
        rates,
        timestamp: chrono::Utc::now(),
        source: Source::Frankfurter,
    };

    let json = serde_json::to_string(&collection).unwrap();
    let deserialized: RateCollection = serde_json::from_str(&json).unwrap();

    assert_eq!(collection.base_code, deserialized.base_code);
    assert_eq!(collection.rates.len(), deserialized.rates.len());
}

#[test]
fn historical_rate_serialization() {
    let mut rates = std::collections::HashMap::new();
    rates.insert("USD".to_string(), 1.09);
    rates.insert("GBP".to_string(), 0.86);

    let historical = HistoricalRate {
        base_code: "EUR".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        rates,
        source: Source::Frankfurter,
    };

    let json = serde_json::to_string(&historical).unwrap();
    let deserialized: HistoricalRate = serde_json::from_str(&json).unwrap();

    assert_eq!(historical.base_code, deserialized.base_code);
    assert_eq!(historical.date, deserialized.date);
}

#[test]
fn currency_metadata_serialization() {
    let metadata = CurrencyMetadata {
        code: "USD".to_string(),
        locale: "en-US".to_string(),
        two_letter_country_code: "US".to_string(),
        currency_name: "US Dollar".to_string(),
        currency_name_short: "Dollar".to_string(),
        display_symbol: "$".to_string(),
        flag_url: "https://example.com/flags/us.png".to_string(),
    };

    let json = serde_json::to_string(&metadata).unwrap();
    let deserialized: CurrencyMetadata = serde_json::from_str(&json).unwrap();

    assert_eq!(metadata.code, deserialized.code);
    assert_eq!(metadata.locale, deserialized.locale);
}

#[test]
fn api_response_types_serialization() {
    let success = ResponseResult::Success;
    let error = ResponseResult::Error;

    assert_eq!(
        "success",
        serde_json::to_string(&success).unwrap().trim_matches('"')
    );
    assert_eq!(
        "error",
        serde_json::to_string(&error).unwrap().trim_matches('"')
    );
}

#[test]
fn error_type_serialization() {
    let error_types = vec![
        (ErrorType::MissingKey, "missing-key"),
        (ErrorType::InvalidKey, "invalid-key"),
        (ErrorType::InactiveAccount, "inactive-account"),
        (ErrorType::QuotaReached, "quota-reached"),
        (ErrorType::NotFound, "not-found"),
        (ErrorType::InvalidCurrency, "invalid-currency"),
        (ErrorType::InvalidDate, "invalid-date"),
        (ErrorType::MalformedRequest, "malformed-request"),
    ];

    for (error_type, expected) in error_types {
        let json = serde_json::to_string(&error_type).unwrap();
        assert!(json.contains(expected));
    }
}

#[test]
fn error_response_serialization() {
    let error = ErrorResponse {
        result: ResponseResult::Error,
        error_type: ErrorType::InvalidCurrency,
        message: Some("Unknown currency code: XYZ".to_string()),
    };

    let json = serde_json::to_string(&error).unwrap();
    assert!(json.contains("error"));
    assert!(json.contains("invalid-currency"));
    assert!(json.contains("Unknown currency code"));
}

#[test]
fn latest_rates_response_serialization() {
    let mut rates = std::collections::HashMap::new();
    rates.insert("EUR".to_string(), 0.92);
    rates.insert("GBP".to_string(), 0.79);

    let data_source = DataSourceInfo {
        source: "test".to_string(),
        last_retrieved: "2023-11-15T00:00:00Z".to_string(),
        last_cached: None,
        upstream_request: UpstreamRequestInfo {
            endpoint: "https://test.example.com/rates".to_string(),
            method: None,
            headers: None,
            payload: None,
        },
    };

    let response = LatestRatesResponse {
        result: ResponseResult::Success,
        documentation: "https://example.com/docs".to_string(),
        time_last_update_unix: 1700000000,
        time_last_update_utc: "2023-11-15T00:00:00Z".to_string(),
        time_next_update_unix: 1700086400,
        time_next_update_utc: "2023-11-16T00:00:00Z".to_string(),
        base_code: "USD".to_string(),
        conversion_rates: rates,
        data_source,
    };

    let json = serde_json::to_string(&response).unwrap();
    let deserialized: LatestRatesResponse = serde_json::from_str(&json).unwrap();

    assert_eq!(response.base_code, deserialized.base_code);
    assert_eq!(
        response.conversion_rates.len(),
        deserialized.conversion_rates.len()
    );
}

#[test]
fn pair_response_serialization() {
    let data_source = DataSourceInfo {
        source: "test".to_string(),
        last_retrieved: "2023-11-15T00:00:00Z".to_string(),
        last_cached: Some("2023-11-15T00:05:00Z".to_string()),
        upstream_request: UpstreamRequestInfo {
            endpoint: "https://test.example.com/pair".to_string(),
            method: None,
            headers: None,
            payload: None,
        },
    };

    let response = PairResponse {
        result: ResponseResult::Success,
        documentation: "https://example.com/docs".to_string(),
        time_last_update_unix: 1700000000,
        time_last_update_utc: "2023-11-15T00:00:00Z".to_string(),
        time_next_update_unix: 1700086400,
        time_next_update_utc: "2023-11-16T00:00:00Z".to_string(),
        base_code: "USD".to_string(),
        target_code: "EUR".to_string(),
        conversion_rate: 0.92,
        conversion_result: Some(92.0),
        data_source,
    };

    let json = serde_json::to_string(&response).unwrap();
    let deserialized: PairResponse = serde_json::from_str(&json).unwrap();

    assert_eq!(response.base_code, deserialized.base_code);
    assert_eq!(response.target_code, deserialized.target_code);
    assert!((response.conversion_rate - deserialized.conversion_rate).abs() < f64::EPSILON);
}

#[test]
fn validation_error_display() {
    let err = ValidationError::InvalidCurrencyCode("XYZ".to_string());
    assert!(err.to_string().contains("XYZ"));

    let err = ValidationError::InvalidDate("invalid".to_string());
    assert!(err.to_string().contains("invalid"));

    let err = ValidationError::InvalidRate("negative".to_string());
    assert!(err.to_string().contains("negative"));
}

#[test]
fn error_into_response() {
    let error = Error::InvalidCurrency("XYZ".to_string());
    let response = error.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
}

#[test]
fn error_status_codes() {
    assert_eq!(
        Error::NotFound("test".to_string()).status_code(),
        axum::http::StatusCode::NOT_FOUND
    );
    assert_eq!(
        Error::Validation("test".to_string()).status_code(),
        axum::http::StatusCode::BAD_REQUEST
    );
    assert_eq!(
        Error::InvalidCurrency("test".to_string()).status_code(),
        axum::http::StatusCode::BAD_REQUEST
    );
    assert_eq!(
        Error::InvalidDate("test".to_string()).status_code(),
        axum::http::StatusCode::BAD_REQUEST
    );
    assert_eq!(
        Error::Internal("test".to_string()).status_code(),
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[test]
fn currencies_response_flatten() {
    let mut currencies = std::collections::HashMap::new();
    currencies.insert("USD".to_string(), "US Dollar".to_string());
    currencies.insert("EUR".to_string(), "Euro".to_string());

    let response = CurrenciesResponse { currencies };

    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("USD"));
    assert!(json.contains("Euro"));
    assert!(!json.contains("currencies"));
}
