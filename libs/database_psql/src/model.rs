use chrono::{DateTime, Utc};
use chrono::NaiveDateTime;
use diesel::{prelude::*, sql_query};
use diesel::sql_types::Timestamp;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::cursors;
use crate::schema::trades;

#[table_name = "cursors"]
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
pub struct Cursor {
    pub id: String,
    pub value: Option<String>,
    pub block: Option<i64>,
}

#[table_name = "trades"]
#[derive(Queryable, QueryableByName, Insertable, Serialize, Deserialize, ToSchema, Debug)]
pub struct Trade {
    pub pk: String,
    pub signature: String,
    pub symbol: String,
    pub block: i64,
    pub timestamp: i64,
    pub timestamp_ts: NaiveDateTime,
    pub order_taker: String,
    pub order_initializer: String,
    pub currency_mint: String,
    pub asset_mint: String,
    pub asset_receiving_wallet: String,
    pub asset_change: f64,
    pub market_fee: f64,
    pub total_cost: f64,
    pub price: f64,
}


// #[table_name = "trades"]
// #[derive(Queryable, QueryableByName, Insertable, Serialize, Deserialize, ToSchema, Debug)]
// struct Volume {
//     day: String,
//     volume: f32,
// }


