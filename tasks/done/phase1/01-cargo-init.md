# Task: Initialize Cargo Project

## Status
[ ] Pending

## Description

Initialize the Rust project with Cargo and add all required dependencies.

## Dependencies to Add

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["trace", "cors"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
config = "0.14"
secrecy = { version = "0.8", features = ["serde"] }
thiserror = "1"

[dev-dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio-test = "0.4"
```

## Files
- `Cargo.toml`

## Notes
- Use edition 2021
- Set up release profile optimizations
