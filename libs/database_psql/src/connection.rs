use std::env;
use diesel::{Connection, PgConnection, RunQueryDsl};
use crate::model::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn create_psql_pool() -> Pool<ConnectionManager<PgConnection>> {
    //GET env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    //Create connection manager
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);

    //Create
    let pool = Pool::builder()
        .max_size(10)
        .build(connection_manager)
        .expect("Failed to create pool");
    pool
}
