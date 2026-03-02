//! SlowPokeAPI - Currency exchange rate API with distributed sync

pub mod config;
pub mod handlers;
pub mod models;
pub mod server;

pub use config::Settings;
pub use models::error::{Error, Result};
