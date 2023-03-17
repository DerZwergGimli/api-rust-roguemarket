use std::env;
use std::str::FromStr;

use deadpool_postgres::{Manager, ManagerConfig, Pool as DeadPool, RecyclingMethod};
use deadpool_postgres::Config;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use tokio_postgres::NoTls;

pub fn create_psql_pool_diesel() -> Pool<ConnectionManager<PgConnection>> {
    //GET env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    //Create connection manager
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);

    //Create
    let pool = Pool::builder()
        .max_size(5)
        .build(connection_manager)
        .expect("Failed to create pool");
    pool
}

pub fn create_psql_raw_pool() -> deadpool_postgres::Pool {
    //GET env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    //Create connection manager

    tokio_postgres::Config::from_str(database_url.as_str()).unwrap();
    let mut cfg = tokio_postgres::Config::from_str(database_url.as_str()).unwrap();


    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };
    let connection_manager = Manager::from_config(cfg, NoTls, mgr_config);

    //Create
    let pool = DeadPool::builder(connection_manager).max_size(16).build().unwrap();
    pool
}
