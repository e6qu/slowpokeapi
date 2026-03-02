use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CurrencyType {
    Fiat,
    Crypto,
    Metal,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Currency {
    pub code: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    pub currency_type: CurrencyType,
}

impl Currency {
    pub fn is_fiat(&self) -> bool {
        matches!(self.currency_type, CurrencyType::Fiat)
    }

    pub fn is_crypto(&self) -> bool {
        matches!(self.currency_type, CurrencyType::Crypto)
    }

    pub fn is_metal(&self) -> bool {
        matches!(self.currency_type, CurrencyType::Metal)
    }
}
