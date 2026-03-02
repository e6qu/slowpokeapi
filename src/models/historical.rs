use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use super::Source;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HistoricalRate {
    pub base_code: String,
    pub date: NaiveDate,
    pub rates: HashMap<String, f64>,
    pub source: Source,
}
