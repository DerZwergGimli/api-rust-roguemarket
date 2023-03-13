use std::env;
use diesel::{Connection, PgConnection, RunQueryDsl};
use crate::model::*;
use crate::schema::*;
use diesel::prelude::*;


pub fn create_or_update_trade_table(connection: &mut PgConnection, data: Trade) {
    use crate::schema::trades::dsl::*;
    use crate::schema::trades;
    //Check if Trade is already in DB
    let trades_vec: Vec<Trade> = trades
        .filter(signature.eq(data.signature.clone()))
        .limit(1)
        .load::<Trade>(connection)
        .expect("Unable to trades form db!");


    if trades_vec.len() > 0 {
        //TODO: make the data update
    } else {
        diesel::insert_into(trades::table)
            .values(&data)
            .get_result::<Trade>(connection)
            .expect("Error inserting trade into DB!");
    }
}