use std::convert::Infallible;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Error, Pool};
use warp::{Filter, reject};
use database_psql::connection::create_psql_pool;


pub fn with_psql_store(db_pool: Pool<ConnectionManager<PgConnection>>) -> impl Filter<Extract=(Pool<ConnectionManager<PgConnection>>, ), Error=Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

