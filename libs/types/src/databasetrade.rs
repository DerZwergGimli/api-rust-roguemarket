use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DBTrade {
    pub signature: String,
    pub timestamp: i64,
    pub slot: u64,
    pub symbol: String,
    pub exchange: Option<Vec<Exchange>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Exchange {
    pub side: bool,
    pub seller: String,
    pub buyer: String,
    pub currency_mint: String,
    pub token_mint: String,
    pub currency_amount: f64,
    pub token_amount: f64,
}
