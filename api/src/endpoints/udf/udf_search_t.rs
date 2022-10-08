use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UdfSearchSymbol {
    pub symbol: String,
    pub full_name: String,
    pub description: String,
    pub exchange: String,
    pub ticker: String,
    #[serde(rename = "type")]
    pub udf_symbol_type: String,
}
