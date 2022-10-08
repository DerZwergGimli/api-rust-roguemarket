use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UdfSymbols {
    pub symbol: String,
    pub ticker: String,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub exchange: String,
    pub listed_exchange: String,
    #[serde(rename = "type")]
    pub udf_symbols_type: String,
    pub currency_code: String,
    pub session: String,
    pub timezone: String,
    pub minmovement: i64,
    pub minmov: i64,
    pub minmovement2: i64,
    pub minmov2: i64,
    pub pricescale: i64,
    pub supported_resolutions: Vec<String>,
    pub has_intraday: bool,
    pub has_daily: bool,
    pub has_weekly_and_monthly: bool,
    pub data_status: String,
}
