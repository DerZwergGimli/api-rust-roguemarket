use std::{
    convert::Infallible,
    env,
    sync::{Arc, Mutex},
};
use std::future::Future;
use std::time::{SystemTime, UNIX_EPOCH};

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use staratlas::symbolstore::{BuilderSymbolStore, SymbolStore};
use types::databasetrade::DBTrade;
use types::m_ohclvt::M_OHCLVT;
use utoipa::{IntoParams, ToSchema};
use warp::{Filter, hyper::StatusCode, Reply};
use warp::sse::reply;

use database_psql::connection::create_psql_pool_diesel;
use database_psql::model::Trade;

use crate::endpoints::responses::response_error::ResponseError;
use crate::endpoints::trades::trades;
use crate::endpoints::udf::{udf_config_t, udf_history_t, udf_symbols_t};
use crate::endpoints::udf::{udf_search_t, udf_symbol_info_t};
use crate::endpoints::udf::udf_error_t::{Status, UdfError};
use crate::helper::with_psql_store;
use crate::udf_config_t::{Exchange, SymbolsType};

//region PARAMS


//endregion

//region HANDLERS
pub async fn handlers() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone
{
    let psql_pool = create_psql_pool_diesel();

    let last_timestamp = warp::path!("stats"  / "last_timestamp")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_psql_store(psql_pool.clone()))
        .and_then(get_last_timestamp);

    let first_timestamp = warp::path!("stats"  / "first_timestamp")
        .and(warp::get())
        .and(with_psql_store(psql_pool.clone()))
        .and(warp::path::end())
        .and_then(get_first_timestamp);

    let ranges = warp::path!("stats"  / "ranges")
        .and(warp::get())
        .and(with_psql_store(psql_pool.clone()))
        .and(warp::path::end())
        .and_then(get_ranges);

    last_timestamp.or(first_timestamp).or(ranges)
}
//endregion

/// Last timestamp
///
/// Responses with logged trade timestamp.
#[utoipa::path(
get,
path = "/stats/last_timestamp",
responses(
(status = 200, description = "Get time successfully", body = [Trade])
)
)]
pub async fn get_last_timestamp(db_pool: Pool<ConnectionManager<PgConnection>>) -> Result<impl Reply, Infallible> {
    let mut db = db_pool.get().unwrap();

    use diesel::prelude::*;
    use database_psql::model::*;
    use database_psql::schema::trades::dsl::*;

    let cursor_db: Vec<Trade> = trades
        .order(timestamp.desc())
        .limit(1)
        .load::<Trade>(&mut db)
        .expect("Error loading cursors");

    return if cursor_db.is_empty() {
        warn!("While getting get_first_timestamp");
        Ok(warp::reply::json(&ResponseError {
            s: 1,
            errmsg: "No data found".to_string(),
        }))
    } else {
        Ok(warp::reply::json(&cursor_db))
    };
}

/// First timestamp
///
/// Responses with logged trade timestamp.
#[utoipa::path(
get,
path = "/stats/first_timestamp",
responses(
(status = 200, description = "Get time successfully", body = [Trade])
)
)]
pub async fn get_first_timestamp(db_pool: Pool<ConnectionManager<PgConnection>>) -> Result<impl Reply, Infallible> {
    let mut db = db_pool.get().unwrap();

    use diesel::prelude::*;
    use database_psql::model::*;
    use database_psql::schema::trades::dsl::*;

    let cursor_db: Vec<Trade> = trades
        .order(timestamp.asc())
        .limit(1)
        .load::<Trade>(&mut db)
        .expect("Error loading cursors");

    return if cursor_db.is_empty() {
        warn!("While getting get_first_timestamp");
        Ok(warp::reply::json(&ResponseError {
            s: 1,
            errmsg: "No data found".to_string(),
        }))
    } else {
        Ok(warp::reply::json(&cursor_db))
    };
}


/// Ranges
///
/// Responses with sync status ranges
#[utoipa::path(
get,
path = "/stats/ranges",
responses(
(status = 200, description = "Get range successfully", body = [Cursor])
)
)]
pub async fn get_ranges(db_pool: Pool<ConnectionManager<PgConnection>>) -> Result<impl Reply, Infallible> {
    let mut db = db_pool.get().unwrap();

    use diesel::prelude::*;
    use database_psql::model::*;
    use database_psql::schema::cursors::dsl::*;

    let cursor_db: Vec<Cursor> = cursors
        .order(id.desc())
        .load::<Cursor>(&mut db)
        .expect("Error loading cursors");

    return if cursor_db.is_empty() {
        warn!("While requesting get_ranges");
        Ok(warp::reply::json(&ResponseError {
            s: 1,
            errmsg: "No data found".to_string(),
        }))
    } else {
        Ok(warp::reply::json(&cursor_db))
    };
}

