use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DBTrade {
    pub signature: String,
    pub timestamp: i64,
    pub assetMint: String,
    pub orderTaker: String,
    pub orderInitializer: String,
    pub size: f64,
    pub price: f64,
    pub cost: f64,
    pub pair: String,
}
