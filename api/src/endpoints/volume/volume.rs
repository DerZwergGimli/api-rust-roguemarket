use std::{
    convert::Infallible,
    env,
    sync::{Arc, Mutex},
};
use std::fmt::format;
use std::future::Future;
use std::mem::swap;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, NaiveDate, Utc};
use database_psql::connection::{create_psql_pool_diesel, create_psql_raw_pool};
use database_psql::model::Trade;
use database_psql::schema::trades::timestamp;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, sql_query};
use diesel::dsl::date;
use diesel::r2d2::{ConnectionManager, Pool};
use log::{info, warn};
use postgres::{NoTls, Row};
use r2d2_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use staratlas::symbolstore::{BuilderSymbolStore, SymbolStore};
use types::databasetrade::DBTrade;
use types::m_ohclvt::M_OHCLVT;
use utoipa::{IntoParams, ToSchema};
use warp::{Filter, hyper::StatusCode, Reply};
use warp::sse::reply;

use crate::endpoints::responses::response_error::ResponseError;
use crate::endpoints::responses::response_trade::create_response;
use crate::endpoints::udf::{udf_config_t, udf_history_t, udf_symbols_t};
use crate::endpoints::udf::{udf_search_t, udf_symbol_info_t};
use crate::endpoints::udf::udf::get_history;
use crate::endpoints::udf::udf_error_t::{Status, UdfError};
use crate::helper::{with_psql_store, with_raw_psql_store};
use crate::udf_config_t::{Exchange, SymbolsType};

//region PARAMS


#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultVolumeParams {
    wallet: Option<String>,
    currency: Option<String>,
    asset: Option<String>,
    from: Option<i64>,
}

//endregion

//region HANDLERS
pub async fn handlers() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone
{
    let psql_raw_pool = create_psql_raw_pool();
    let psql_pool = create_psql_pool_diesel();


    let volume_total = warp::path!("volume" / "total")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_psql_store(psql_pool.clone()))
        .and(warp::query::<DefaultVolumeParams>())
        .and_then(get_volume_total);


    let volume_history = warp::path!("volume" / "history")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_psql_store(psql_pool.clone()))
        .and(warp::query::<DefaultVolumeParams>())
        .and_then(get_volume_history);


    volume_total.or(volume_history)
}


//endregion


/// Get last trade from SYMBOL
///
/// Responses with a last trade for a given symbol. [default 10]
#[utoipa::path(
get,
path = "/volume/total",
params(DefaultVolumeParams),
responses(
(status = 200, description = "Response: Time successful", body = [String])
)
)]
pub async fn get_volume_total(
    db_pool: Pool<ConnectionManager<PgConnection>>,
    query: DefaultVolumeParams,
) -> Result<impl Reply, Infallible> {
    let mut db = db_pool.get().expect("Unable to get connection from pool!");
    use diesel::prelude::*;
    use database_psql::model::*;
    use database_psql::schema::trades::dsl::*;
    use diesel::dsl::*;

    let cursor_db = match query.wallet {
        None => {
            match query.currency {
                None => {
                    match query.asset {
                        None => {
                            vec![]
                        }
                        Some(asset) => {
                            trades
                                .filter(asset_mint.like(format!("%{}%", asset.clone())))
                                .select(sum(asset_change * price))
                                .load::<Trade>(&mut db)
                                .expect("Error loading cursors")
                        }
                    }
                }
                Some(_) => {
                    match query.asset {
                        None => {
                            trades
                                .filter(symbol.like(format!("%{}%", query.symbol.clone())))
                                .order(timestamp.desc())
                                .limit(query.limit.unwrap_or(10))
                                .load::<Trade>(&mut db)
                                .expect("Error loading cursors")
                        }
                        Some(_) => {
                            trades
                                .filter(symbol.like(format!("%{}%", query.symbol.clone())))
                                .order(timestamp.desc())
                                .limit(query.limit.unwrap_or(10))
                                .load::<Trade>(&mut db)
                                .expect("Error loading cursors")
                        }
                    }
                }
            }
        }
        Some(_) => {
            match query.currency {
                None => {
                    match query.asset {
                        None => {
                            trades
                                .filter(symbol.like(format!("%{}%", query.symbol.clone())))
                                .order(timestamp.desc())
                                .limit(query.limit.unwrap_or(10))
                                .load::<Trade>(&mut db)
                                .expect("Error loading cursors")
                        }
                        Some(_) => {
                            trades
                                .filter(symbol.like(format!("%{}%", query.symbol.clone())))
                                .order(timestamp.desc())
                                .limit(query.limit.unwrap_or(10))
                                .load::<Trade>(&mut db)
                                .expect("Error loading cursors")
                        }
                    }
                }
                Some(_) => {
                    match query.asset {
                        None => {
                            trades
                                .filter(symbol.like(format!("%{}%", query.symbol.clone())))
                                .order(timestamp.desc())
                                .limit(query.limit.unwrap_or(10))
                                .load::<Trade>(&mut db)
                                .expect("Error loading cursors")
                        }
                        Some(_) => {
                            trades
                                .filter(symbol.like(format!("%{}%", query.symbol.clone())))
                                .order(timestamp.desc())
                                .limit(query.limit.unwrap_or(10))
                                .load::<Trade>(&mut db)
                                .expect("Error loading cursors")
                        }
                    }
                }
            }
        }
    };

    // let cursor_db: Vec<Trade> = match query.to {
    //     None => {
    //         trades
    //             .filter(symbol.like(format!("%{}%", query.symbol.clone())))
    //             .order(timestamp.desc())
    //             .limit(query.limit.unwrap_or(10))
    //             .load::<Trade>(&mut db)
    //             .expect("Error loading cursors")
    //     }
    //     Some(to) => {
    //         trades
    //             .filter(symbol.like(format!("%{}%", query.symbol.clone()))
    //                 .and(timestamp.le(to)))
    //             .order(timestamp.desc())
    //             .limit(query.limit.unwrap_or(10))
    //             .load::<Trade>(&mut db)
    //             .expect("Error loading cursors")
    //     }
    // };
    // create_response(&cursor_db)
    //
    Ok(warp::reply::json(&ResponseError {
        s: 1,
        errmsg: "No data found".to_string(),
    }))
}


/// Get last trade from SYMBOL
///
/// Responses with a last trade for a given symbol. [default 10]
#[utoipa::path(
get,
path = "/volume/history",
params(DefaultVolumeParams),
responses(
(status = 200, description = "Response: Time successful", body = [String])
)
)]
pub async fn get_volume_history(
    db_pool: Pool<ConnectionManager<PgConnection>>,
    query: DefaultVolumeParams,
) -> Result<impl Reply, Infallible> {
    let mut db = db_pool.get().expect("Unable to get connection from pool!");
    use diesel::prelude::*;
    use database_psql::model::*;
    use database_psql::schema::trades::dsl::*;
    //
    // let cursor_db: Vec<Trade> = match query.to {
    //     None => {
    //         trades
    //             .filter(symbol.like(format!("%{}%", query.symbol.clone())))
    //             .order(timestamp.desc())
    //             .limit(query.limit.unwrap_or(10))
    //             .load::<Trade>(&mut db)
    //             .expect("Error loading cursors")
    //     }
    //     Some(to) => {
    //         trades
    //             .filter(symbol.like(format!("%{}%", query.symbol.clone()))
    //                 .and(timestamp.le(to)))
    //             .order(timestamp.desc())
    //             .limit(query.limit.unwrap_or(10))
    //             .load::<Trade>(&mut db)
    //             .expect("Error loading cursors")
    //     }
    // };
    // create_response(&cursor_db)
    Ok(warp::reply::json(&ResponseError {
        s: 1,
        errmsg: "No data found".to_string(),
    }))
}


