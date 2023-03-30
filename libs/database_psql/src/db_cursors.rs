use diesel::{PgConnection, RunQueryDsl};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::model::*;
use crate::schema::*;

pub fn get_cursor(connection: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>, name_id: String) -> Vec<Cursor> {
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
        .set((value.eq(new_cursor.value), block.eq(new_cursor.block), block.eq(new_cursor.start_block), block.eq(new_cursor.end_block)))
        .get_result::<Cursor>(connection)
        .expect("Error updating Cursor!");
    return cursor_db;
}
