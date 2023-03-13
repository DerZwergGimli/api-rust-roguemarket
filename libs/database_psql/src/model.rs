use diesel::prelude::*;

use crate::schema::cursors;
use crate::schema::trades;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
//#[table_name = "cursors"]
pub struct Cursor {
    pub id: String,
    pub value: Option<String>,
    pub block: Option<i64>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
pub struct Trade {
    pub signature: String,
    pub symbol: String,
    pub block: i64,
    pub timestamp: i64,
    pub order_taker: String,
    pub order_initializer: String,
    pub currency_mint: String,
    pub asset_mint: String,
    pub asset_change: f64,
    pub market_fee: f64,
    pub total_cost: f64,
}

