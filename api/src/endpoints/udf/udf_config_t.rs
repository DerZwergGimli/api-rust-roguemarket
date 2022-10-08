use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UdfConfig {
    pub exchanges: Vec<Exchange>,
    pub symbols_types: Vec<SymbolsType>,
    pub supported_resolutions: Vec<String>,
    pub supports_search: bool,
    pub supports_group_request: bool,
    pub supports_marks: bool,
    pub supports_timescale_marks: bool,
    pub supports_time: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Exchange {
    pub value: String,
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct SymbolsType {
    pub value: String,
    pub name: String,
}
