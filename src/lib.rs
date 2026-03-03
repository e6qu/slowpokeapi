//! SlowPokeAPI - Currency exchange rate API with distributed sync

pub mod cache;
pub mod config;
pub mod handlers;
pub mod metrics;
pub mod models;
pub mod server;
pub mod storage;
pub mod upstream;
pub use config::Settings;
pub use models::error::{Error, Result};
