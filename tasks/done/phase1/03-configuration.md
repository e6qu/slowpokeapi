# Task: Implement Configuration

## Status
[ ] Pending

## Description

Implement configuration loading from environment variables and optional config file.

## Requirements

1. Configuration struct with all settings
2. Load from environment variables (prefix: `SLOWPOKEAPI_`)
3. Support optional config file
4. Default values for optional settings

## Settings

```rust
struct Settings {
    server: ServerConfig,
    logging: LoggingConfig,
}

struct ServerConfig {
    host: String,        // default: "0.0.0.0"
    port: u16,           // default: 8080
}

struct LoggingConfig {
    level: String,       // default: "info"
    format: String,      // default: "json"
}
```

## Files
- `src/config/mod.rs`
- `src/config/settings.rs`

## Notes
- Use `config` crate for loading
- Validate configuration on startup
