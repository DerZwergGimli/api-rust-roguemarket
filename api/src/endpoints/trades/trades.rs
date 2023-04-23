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
use crate::endpoints::udf::udf_error_t::{Status, UdfError};
use crate::helper::{with_psql_store, with_raw_psql_store};
use crate::udf_config_t::{Exchange, SymbolsType};

//region PARAMS
#[derive(Debug, Serialize, ToSchema)]
pub struct VolumeData {
    #[schema(value_type = String)]
    time: NaiveDate,
    volume: f64,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultBaseParams {
    limit: Option<i64>,
    to: Option<i64>,
    from: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultSymbolParams {
    #[param(style = Form, example = "FOODATLAS")]
    symbol: String,
    limit: Option<i64>,
    to: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultSignatureParams {
    signature: String,
    limit: Option<i64>,
    to: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultAddressParams {
    address: String,
    limit: Option<i64>,
    to: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultMintParams {
    asset_mint: String,
    currency_mint: Option<String>,
    limit: Option<i64>,
    to: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultVolumeParams {
    #[param(style = Form, example = "ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx")]
    currency_mint: String,
    asset_mint: Option<String>,
    #[param(style = Form, example = "1659164001")]
    from: i64,
}

//endregion

//region HANDLERS
pub async fn handlers() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone
{
    let psql_raw_pool = create_psql_raw_pool();
    let psql_pool = create_psql_pool_diesel();

    let base = warp::path!("trades")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_psql_store(psql_pool.clone()))
        .and(warp::query::<DefaultBaseParams>())
        .and_then(get_base);


    let info = warp::path!("trades" / "symbol")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_psql_store(psql_pool.clone()))
        .and(warp::query::<DefaultSymbolParams>())
        .and_then(get_symbol);

    let signature = warp::path!("trades" / "signature")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_psql_store(psql_pool.clone()))
        .and(warp::query::<DefaultSignatureParams>())
        .and_then(get_signature);

    let mint = warp::path!("trades" / "mint")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_psql_store(psql_pool.clone()))
        .and(warp::query::<DefaultMintParams>())
        .and_then(get_mint);

    let address = warp::path!("trades" / "address")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_psql_store(psql_pool.clone()))
        .and(warp::query::<DefaultAddressParams>())
        .and_then(get_address);

    let volume = warp::path!("trades" / "volume")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_raw_psql_store(psql_raw_pool.clone()))
        .and(warp::query::<DefaultVolumeParams>())
        .and_then(get_volume);

    base.or(info).or(signature).or(mint).or(address).or(volume)
}


//endregion

/// Get x trade from Database
///
/// Responses with a last trade for a given symbol. [default 10]
#[utoipa::path(
get,
path = "/trades",
params(DefaultBaseParams),
responses(
(status = 200, description = "Response: Time successful", body = [Trade])
)
)]
pub async fn get_base(
    db_pool: Pool<ConnectionManager<PgConnection>>,
    query: DefaultBaseParams,
) -> Result<impl Reply, Infallible> {
    let mut db = db_pool.get().expect("Unable to get connection from pool!");
    use diesel::prelude::*;
    use database_psql::model::*;
    use database_psql::schema::trades::dsl::*;

    let cursor_db: Vec<Trade> = match query.to {
        None => {
            match query.from {
                None => {
                    trades
                        .order(timestamp.desc())
                        .limit(query.limit.unwrap_or(10))
                        .load::<Trade>(&mut db)
                        .expect("Error loading trades")
                }
                Some(from) => {
                    trades
                        .filter(timestamp.ge(from))
                        .order(timestamp.desc())
                        .limit(query.limit.unwrap_or(10))
                        .load::<Trade>(&mut db)
                        .expect("Error loading trades")
                }
            }
        }
        Some(to) => {
            match query.from {
                None => {
                    trades
                        .filter(timestamp.le(to))
                        .order(timestamp.desc())
                        .limit(query.limit.unwrap_or(10))
                        .load::<Trade>(&mut db)
                        .expect("Error loading trades")
                }
                Some(from) => {
                    trades
                        .filter(timestamp.le(to)
                            .and(timestamp.ge(from)))
                        .order(timestamp.desc())
                        .limit(query.limit.unwrap_or(10))
                        .load::<Trade>(&mut db)
                        .expect("Error loading trades")
                }
            }
        }
    };
    create_response(&cursor_db)
}


/// Get last trade from SYMBOL
///
/// Responses with a last trade for a given symbol. [default 10]
#[utoipa::path(
get,
path = "/trades/symbol",
params(DefaultSymbolParams),
responses(
(status = 200, description = "Response: Time successful", body = [Trade])
)
)]
pub async fn get_symbol(
    db_pool: Pool<ConnectionManager<PgConnection>>,
    query: DefaultSymbolParams,
) -> Result<impl Reply, Infallible> {
    let mut db = db_pool.get().expect("Unable to get connection from pool!");
    use diesel::prelude::*;
    use database_psql::model::*;
    use database_psql::schema::trades::dsl::*;

    let cursor_db: Vec<Trade> = match query.to {
        None => {
            trades
                .filter(symbol.like(format!("%{}%", query.symbol.clone())))
                .order(timestamp.desc())
                .limit(query.limit.unwrap_or(10))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
        Some(to) => {
            trades
                .filter(symbol.like(format!("%{}%", query.symbol.clone()))
                    .and(timestamp.le(to)))
                .order(timestamp.desc())
                .limit(query.limit.unwrap_or(10))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
    };
    create_response(&cursor_db)
}


/// Get trade for signature
///
/// Responses with a trade for a given signature.
#[utoipa::path(
get,
path = "/trades/signature",
params(DefaultSignatureParams),
responses(
(status = 200, description = "Response: Time successful", body = [Trade])
)
)]
pub async fn get_signature(
    db_pool: Pool<ConnectionManager<PgConnection>>,
    query: DefaultSignatureParams,
) -> Result<impl Reply, Infallible> {
    let mut db = db_pool.get().expect("Unable to get connection from pool!");

    use diesel::prelude::*;
    use database_psql::model::*;
    use database_psql::schema::trades::dsl::*;

    let cursor_db: Vec<Trade> = match query.to {
        None => {
            trades
                .filter(symbol.like(format!("%{}%", query.signature.clone())))
                .order(timestamp.desc())
                .limit(query.limit.unwrap_or(10))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
        Some(to) => {
            trades
                .filter(symbol.like(format!("%{}%", query.signature.clone()))
                    .and(timestamp.le(to)))
                .order(timestamp.desc())
                .limit(query.limit.unwrap_or(10))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
    };

    create_response(&cursor_db)
}


/// Get trade for address
///
/// Responses with an array of trades for buy/sell-wallet-address.
#[utoipa::path(
get,
path = "/trades/address",
params(DefaultAddressParams),
responses(
(status = 200, description = "Response: Time successful", body = [Trade])
)
)]
pub async fn get_address(
    db_pool: Pool<ConnectionManager<PgConnection>>,
    query: DefaultAddressParams,
) -> Result<impl Reply, Infallible> {
    let mut db = db_pool.get().expect("Unable to get connection from pool!");

    use diesel::prelude::*;
    use database_psql::model::*;
    use database_psql::schema::trades::dsl::*;

    let cursor_db: Vec<Trade> = match query.to {
        None => {
            trades
                .filter(order_taker.like(format!("%{}%", query.address.clone())).or(order_initializer.like(format!("%{}%", query.address.clone()))))
                .order(timestamp.desc())
                .limit(query.limit.unwrap_or(100))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
        Some(to) => {
            trades
                .filter(order_taker.like(format!("%{}%", query.address.clone()))
                    .or(order_initializer.like(format!("%{}%", query.address.clone())))
                    .and(timestamp.le(to)))
                .order(timestamp.desc())
                .limit(query.limit.unwrap_or(100))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
    };

    create_response(&cursor_db)
}


/// Get trade for mint
///
/// Responses with an array of trades for asset/token-mint.
#[utoipa::path(
get,
path = "/trades/mint",
params(DefaultMintParams),
responses(
(status = 200, description = "Response: Time successful", body = [Trade])
)
)]
pub async fn get_mint(
    db_pool: Pool<ConnectionManager<PgConnection>>,
    query: DefaultMintParams,
) -> Result<impl Reply, Infallible> {
    let mut db = db_pool.get().expect("Unable to get connection from pool!");

    use diesel::prelude::*;
    use database_psql::model::*;
    use database_psql::schema::trades::dsl::*;

    let cursor_db: Vec<Trade> = match query.currency_mint {
        None => {
            match query.to {
                None => {
                    trades
                        .filter(
                            asset_mint.like(format!("%{}%", query.asset_mint.clone()))
                        )
                        .order(timestamp.desc())
                        .limit(query.limit.unwrap_or(100))
                        .load::<Trade>(&mut db)
                        .expect("Error loading cursors")
                }
                Some(to) => {
                    trades
                        .filter(
                            asset_mint.like(format!("%{}%", query.asset_mint.clone()))
                                .and(timestamp.le(to))
                        )
                        .order(timestamp.desc())
                        .limit(query.limit.unwrap_or(100))
                        .load::<Trade>(&mut db)
                        .expect("Error loading cursors")
                }
            }
        }
        Some(currency) => {
            match query.to {
                None => {
                    trades
                        .filter(
                            asset_mint.like(format!("%{}%", query.asset_mint.clone()))
                                .and(currency_mint.like(format!("%{}%", currency))))
                        .order(timestamp.desc())
                        .limit(query.limit.unwrap_or(100))
                        .load::<Trade>(&mut db)
                        .expect("Error loading cursors")
                }
                Some(to) => {
                    trades
                        .filter(
                            asset_mint.like(format!("%{}%", query.asset_mint.clone()))
                                .and(currency_mint.like(format!("%{}%", currency)))
                                .and(timestamp.le(to)))
                        .order(timestamp.desc())
                        .limit(query.limit.unwrap_or(100))
                        .load::<Trade>(&mut db)
                        .expect("Error loading cursors")
                }
            }
        }
    };
    create_response(&cursor_db)
}


/// Get trade for mint
///
/// Responses with an array of trades for asset/token-mint.
#[utoipa::path(
get,
path = "/trades/volume",
params(DefaultVolumeParams),
responses(
(status = 200, description = "Response: Time successful", body = [VolumeData])
)
)]
pub async fn get_volume(
    pool: deadpool_postgres::Pool,
    query: DefaultVolumeParams,
) -> Result<impl Reply, Infallible> {
    let mut db = pool.get().await.expect("Unable to get connection from pool!");


    use diesel::{prelude::*, sql_types::*};
    use database_psql::model::*;
    use database_psql::schema::trades::dsl::*;

    let mut volume_data: Vec<VolumeData> = vec![];


    let data: Vec<Row> = match query.asset_mint {
        None => {
            db.query("SELECT date(timestamp_ts) as timestamp,  sum(price*asset_change) as volume
                                            from trades
                                            WHERE (currency_mint LIKE $1) AND timestamp > $2
                                            GROUP BY date(timestamp_ts)
                                            ORDER BY timestamp ASC",
                     &[&query.currency_mint, &query.from]).await.unwrap_or_default()
        }
        Some(value) => {
            db.query("SELECT date(timestamp_ts) as timestamp,  sum(price*asset_change) as volume
                                            from trades
                                            WHERE (currency_mint LIKE $1 AND asset_mint LIKE $2) AND timestamp > $2
                                            GROUP BY date(timestamp_ts)
                                            ORDER BY timestamp ASC",
                     &[&query.currency_mint, &value, &query.from]).await.unwrap_or_default()
        }
    };


    data.into_iter().for_each(|d| {
        volume_data.push(VolumeData {
            time: d.get(0),
            volume: d.get(1),
        })
    });


    create_response(&volume_data)
}
