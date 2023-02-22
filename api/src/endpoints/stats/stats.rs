use crate::endpoints::udf::udf_error_t::{Status, UdfError};
use crate::endpoints::udf::{udf_config_t, udf_history_t, udf_symbols_t};
use crate::endpoints::udf::{udf_search_t, udf_symbol_info_t};
use crate::udf_config_t::{Exchange, SymbolsType};
use log::{info, warn};
use mongo::mongodb::{find_by_signature, find_by_symbol, find_last_or_frist, find_udf_trade_next, find_udf_trades, MongoDBConnection};
use mongodb::bson::Document;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use staratlas::symbolstore::{BuilderSymbolStore, SymbolStore};
use std::future::Future;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    convert::Infallible,
    env,
    sync::{Arc, Mutex},
};
use types::databasetrade::DBTrade;
use types::m_ohclvt::M_OHCLVT;
use udf::time_convert::convert_udf_time_to_minute;
use utoipa::openapi::SchemaFormat::DateTime;
use utoipa::{IntoParams, ToSchema};
use warp::sse::reply;
use warp::{hyper::StatusCode, Filter, Reply};
use crate::endpoints::stats::stats_error::StatsError;

//region PARAMS


//endregion

//region HANDLERS
pub async fn handlers() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone
{
    let mongo_db =
        MongoDBConnection::new(env::var("MONGOURL").expect("NO MONGOURL").as_str()).await;

    let last_timestamp = warp::path!("stats"  / "last_timestamp")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_mongo_store_stats(mongo_db.collection.clone()))
        .and_then(get_last_timestamp);

    let first_timestamp = warp::path!("stats"  / "first_timestamp")
        .and(warp::get())
        .and(with_mongo_store_stats(mongo_db.collection.clone()))
        .and(warp::path::end())
        .and_then(get_first_timestamp);

    last_timestamp.or(first_timestamp)
}

fn with_mongo_store_stats(
    store: Collection<DBTrade>,
) -> impl Filter<Extract=(Collection<DBTrade>, ), Error=Infallible> + Clone {
    warp::any().map(move || store.clone())
}
//endregion

/// Last timestamp
///
/// Responses with logged trade timestamp.
#[utoipa::path(
get,
path = "/stats/last_timestamp",
responses(
(status = 200, description = "Get time successfully", body = [String])
)
)]
pub async fn get_last_timestamp(trades: Collection<DBTrade>) -> Result<impl Reply, Infallible> {
    return match find_last_or_frist(trades, true).await {
        Some(data) => {
            return Ok(warp::reply::json(&data));
        }
        None => {
            warn!("While getting get_last_timestamp");
            Ok(warp::reply::json(&StatsError {
                s: 1,
                errmsg: "No data found".to_string(),
            }))
        }
    };
}

/// First timestamp
///
/// Responses with logged trade timestamp.
#[utoipa::path(
get,
path = "/stats/first_timestamp",
responses(
(status = 200, description = "Get time successfully", body = [String])
)
)]
pub async fn get_first_timestamp(trades: Collection<DBTrade>) -> Result<impl Reply, Infallible> {
    return match find_last_or_frist(trades, false).await {
        Some(data) => {
            return Ok(warp::reply::json(&data));
        }
        None => {
            warn!("While getting get_first_timestamp");
            Ok(warp::reply::json(&StatsError {
                s: 1,
                errmsg: "No data found".to_string(),
            }))
        }
    };
}

