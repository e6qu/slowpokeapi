use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CurrencyMetadata {
    pub code: String,
    pub locale: String,
    pub two_letter_country_code: String,
    pub currency_name: String,
    pub currency_name_short: String,
    pub display_symbol: String,
    pub flag_url: String,
}
