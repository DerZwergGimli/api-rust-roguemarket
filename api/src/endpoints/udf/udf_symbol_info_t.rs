use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UdfSymbolInfo {
    pub symbol: Vec<String>,
    pub ticker: Vec<String>,
    pub name: Vec<String>,
    pub full_name: Vec<String>,
    pub description: Vec<String>,
    pub exchange: String,
    pub listed_exchange: String,
    #[serde(rename = "type")]
    pub udf_symbol_info_type: String,
    pub currency_code: Vec<String>,
    pub session: String,
    pub timezone: String,
    pub minmovement: f64,
    pub minmov: f64,
    pub minmovement2: f64,
    pub minmov2: f64,
    pub pricescale: Vec<i64>,
    pub supported_resolutions: Vec<String>,
    pub has_intraday: bool,
    pub has_daily: bool,
    pub has_weekly_and_monthly: bool,
    pub data_status: String,
}
