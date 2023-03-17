use std::convert::Infallible;

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Error, Pool};
use postgres::{Client, NoTls};
use r2d2_postgres::PostgresConnectionManager;
use warp::{Filter, reject};

use database_psql::connection::create_psql_pool_diesel;

pub fn with_psql_store(db_pool: Pool<ConnectionManager<PgConnection>>) -> impl Filter<Extract=(Pool<ConnectionManager<PgConnection>>, ), Error=Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn with_raw_psql_store(db_pool: deadpool_postgres::Pool) -> impl Filter<Extract=(deadpool_postgres::Pool, ), Error=Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}



