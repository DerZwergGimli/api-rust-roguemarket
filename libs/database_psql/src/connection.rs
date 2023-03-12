use std::env;
use diesel::{Connection, PgConnection, RunQueryDsl};
use crate::model::*;
use crate::schema::*;
use diesel::prelude::*;

pub fn psql_connect() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_cursor(connection: &mut PgConnection, name_id: String) -> Vec<Cursor> {
    use crate::schema::cursors::dsl::*;
    use crate::schema::cursors;
    let cursor_db: Vec<Cursor> = cursors
        .filter(id.eq(name_id))
        .limit(1)
        .load::<Cursor>(connection)
        .expect("Error loading cursors");
    return cursor_db;
}

pub fn create_cursor(connection: &mut PgConnection, new_cursor: Cursor) -> Cursor {
    use crate::schema::cursors::dsl::*;
    use crate::schema::cursors;
    let cursor_db = diesel::insert_into(cursors::table)
        .values(&new_cursor)
        .get_result::<Cursor>(connection)
        .expect("Error creating cursor!");
    return cursor_db;
}

pub fn update_cursor(connection: &mut PgConnection, name_id: String, new_cursor: Cursor) -> Cursor {
    use crate::schema::cursors::dsl::*;
    use crate::schema::cursors;
    let cursor_db = diesel::update(cursors)
        .filter(id.eq(name_id))
        .set((value.eq(new_cursor.value), block.eq(new_cursor.block)))
        .get_result::<Cursor>(connection)
        .expect("Error updating Cursor!");
    return cursor_db;
}

pub fn create_or_update_trade_table(connection: &mut PgConnection, data: Trade) {
    use crate::schema::trades::dsl::*;
    use crate::schema::trades;
    //Check if Trade is already in DB
    let trades_vec: Vec<Trade> = trades.filter(signature.eq(data.signature.clone())).limit(1).load::<Trade>(connection).expect("Unable to trades form db!");
    if trades_vec.len() > 0 {
        //TODO: make the data update
    } else {
        diesel::insert_into(trades::table)
            .values(&data)
            .get_result::<Trade>(connection)
            .expect("Error inserting trade into DB!");
    }
}