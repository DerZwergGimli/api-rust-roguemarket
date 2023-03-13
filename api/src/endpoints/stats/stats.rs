use crate::endpoints::udf::udf_error_t::{Status, UdfError};
use crate::endpoints::udf::{udf_config_t, udf_history_t, udf_symbols_t};
use crate::endpoints::udf::{udf_search_t, udf_symbol_info_t};
use crate::udf_config_t::{Exchange, SymbolsType};
use log::{info, warn};

use serde::{Deserialize, Serialize};
use staratlas::symbolstore::{BuilderSymbolStore, SymbolStore};
use std::future::Future;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    convert::Infallible,
    env,
    sync::{Arc, Mutex},
};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use types::databasetrade::DBTrade;
use types::m_ohclvt::M_OHCLVT;
use udf::time_convert::convert_udf_time_to_minute;
use utoipa::openapi::SchemaFormat::DateTime;
use utoipa::{IntoParams, ToSchema};
use warp::sse::reply;
use warp::{hyper::StatusCode, Filter, Reply};
use database_psql::connection::create_psql_pool;
use database_psql::model::Trade;
use crate::endpoints::stats::stats_error::StatsError;
use crate::endpoints::trades::trades;
use crate::helper::with_psql_store;

//region PARAMS


//endregion

//region HANDLERS
pub async fn handlers() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone
{
    let psql_pool = create_psql_pool();

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

    last_timestamp.or(first_timestamp)
}

// fn with_mongo_store_stats(
//     store: Collection<DBTrade>,
// ) -> impl Filter<Extract=(Collection<DBTrade>, ), Error=Infallible> + Clone {
//     warp::any().map(move || store.clone())
// }
//endregion

/// Last timestamp
///
/// Responses with logged trade timestamp.
#[utoipa::path(
get,
path = "/stats/last_timestamp",
responses(
(status = 200, description = "Get time successfully", body = [SATrade])
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
        Ok(warp::reply::json(&StatsError {
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
(status = 200, description = "Get time successfully", body = [SATrade])
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
        Ok(warp::reply::json(&StatsError {
            s: 1,
            errmsg: "No data found".to_string(),
        }))
    } else {
        Ok(warp::reply::json(&cursor_db))
    };
}

