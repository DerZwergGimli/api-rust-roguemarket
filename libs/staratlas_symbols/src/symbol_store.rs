use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SymbolStore {
    pub assets: Vec<Asset>,
    pub currencies: Vec<Currency>,
    pub exchange: Exchange,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub asset_name: String,
    pub pair_name: String,
    pub description: String,
    pub asset_type: String,
    pub symbol: String,
    pub mint: String,
    pub pair_mint: String,
    pub pricescale: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Currency {
    pub name: String,
    pub mint: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Exchange {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub asset_type: Vec<String>,
    pub sesstion: String,
    pub timezone: String,
    pub minmovement: f64,
    pub minmov: f64,
    pub minmovement2: f64,
    pub minmov2: f64,
    pub supported_resolutions: Vec<String>,
    pub has_intraday: bool,
    pub has_daily: bool,
    pub has_weekly_and_monthly: bool,
    pub data_status: String,
    pub supports_search: bool,
    pub supports_group_request: bool,
    pub supports_marks: bool,
    pub supports_timescale_marks: bool,
    pub supports_time: bool,
}
