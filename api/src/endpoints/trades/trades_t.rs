use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TradesResponse {
    pub signature: String,
    pub timestamp: i64,
    #[serde(rename = "assetMint")]
    pub asset_mint: String,
    #[serde(rename = "currencyMint")]
    pub currency_mint: String,
    #[serde(rename = "orderTaker")]
    pub order_taker: String,
    #[serde(rename = "orderInitializer")]
    pub order_initializer: String,
    pub size: i64,
    pub price: f64,
    pub cost: f64,
    pub pair: String,
}