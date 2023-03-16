use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

//
// #[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
// pub struct SATrade {
//     pub symbol: String,
//     pub signature: String,
//     pub block: u64,
//     pub timestamp: i64,
//     pub order_taker: String,
//     pub currency_mint: String,
//     pub asset_mint: String,
//     pub order_initializer: String,
//     pub asset_change: f64,
//     pub market_fee: f64,
//     pub total_cost: f64,
// }