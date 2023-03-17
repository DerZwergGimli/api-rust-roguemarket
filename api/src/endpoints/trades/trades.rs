use std::{
    convert::Infallible,
    env,
    sync::{Arc, Mutex},
};
use std::future::Future;
use std::mem::swap;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::NaiveDateTime;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use staratlas::symbolstore::{BuilderSymbolStore, SymbolStore};
use types::databasetrade::DBTrade;
use types::m_ohclvt::M_OHCLVT;
use utoipa::{IntoParams, ToSchema};
use utoipa::openapi::SchemaFormat::DateTime;
use warp::{Filter, hyper::StatusCode, Reply};
use warp::sse::reply;

use database_psql::connection::create_psql_pool;
use database_psql::model::Trade;

use crate::endpoints::responses::response_error::ResponseError;
use crate::endpoints::responses::response_trade::create_trade_response;
use crate::endpoints::udf::{udf_config_t, udf_history_t, udf_symbols_t};
use crate::endpoints::udf::{udf_search_t, udf_symbol_info_t};
use crate::endpoints::udf::udf_error_t::{Status, UdfError};
use crate::helper::with_psql_store;
use crate::udf_config_t::{Exchange, SymbolsType};

//region PARAMS
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultLastParams {
    #[param(style = Form, example = "FOODATLAS")]
    symbol: String,
    limit: Option<i64>,
    to: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultSignatureParams {
    signature: String,
    limit: Option<i64>,
    to: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultAddressParams {
    address: String,
    limit: Option<i64>,
    to: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultMintParams {
    asset_mint: String,
    currency_mint: Option<String>,
    limit: Option<i64>,
    to: Option<NaiveDateTime>,
}

//endregion

//region HANDLERS
pub async fn handlers() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone
{
    let psql_pool = create_psql_pool();

    let info = warp::path!("trades" / "last")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_psql_store(psql_pool.clone()))
        .and(warp::query::<DefaultLastParams>())
        .and_then(get_last);

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

    info.or(signature).or(mint).or(address)
}


//endregion


/// Get last trade from SYMBOL
///
/// Responses with a last trade for a given symbol. [default 10]
#[utoipa::path(
get,
path = "/trades/symbol",
params(DefaultLastParams),
responses(
(status = 200, description = "Response: Time successful", body = [Trade])
)
)]
pub async fn get_last(
    db_pool: Pool<ConnectionManager<PgConnection>>,
    query: DefaultLastParams,
) -> Result<impl Reply, Infallible> {
    let mut db = db_pool.get().expect("Unable to get connection from pool!");
    use diesel::prelude::*;
    use database_psql::model::*;
    use database_psql::schema::trades::dsl::*;

    let cursor_db: Vec<Trade> = match query.to {
        None => {
            trades
                .filter(symbol.like(query.symbol.clone()))
                .limit(query.limit.unwrap_or(10))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
        Some(to) => {
            trades
                .filter(symbol.like(query.symbol.clone())
                    .and(timestamp.le(to)))
                .limit(query.limit.unwrap_or(10))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
    };
    create_trade_response(&cursor_db)
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
                .filter(signature.like(query.signature.clone()))
                .limit(query.limit.unwrap_or(10))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
        Some(to) => {
            trades
                .filter(signature.like(query.signature.clone())
                    .and(timestamp.le(to)))
                .limit(query.limit.unwrap_or(10))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
    };

    create_trade_response(&cursor_db)
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
                .filter(order_taker.like(query.address.clone()).or(order_initializer.like(query.address)))
                .limit(query.limit.unwrap_or(100))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
        Some(to) => {
            trades
                .filter(order_taker.like(query.address.clone())
                    .or(order_initializer.like(query.address))
                    .and(timestamp.le(to)))
                .limit(query.limit.unwrap_or(100))
                .load::<Trade>(&mut db)
                .expect("Error loading cursors")
        }
    };

    create_trade_response(&cursor_db)
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
                            asset_mint.like(query.asset_mint.clone())
                        )
                        .limit(query.limit.unwrap_or(100))
                        .load::<Trade>(&mut db)
                        .expect("Error loading cursors")
                }
                Some(to) => {
                    trades
                        .filter(
                            asset_mint.like(query.asset_mint.clone())
                                .and(timestamp.le(to))
                        )
                        .limit(query.limit.unwrap_or(100))
                        .load::<Trade>(&mut db)
                        .expect("Error loading cursors")
                }
            }
        }
        Some(_) => {
            match query.to {
                None => {
                    trades
                        .filter(
                            asset_mint.like(query.asset_mint.clone())
                                .and(currency_mint.like(query.currency_mint.unwrap())))
                        .limit(query.limit.unwrap_or(100))
                        .load::<Trade>(&mut db)
                        .expect("Error loading cursors")
                }
                Some(to) => {
                    trades
                        .filter(
                            asset_mint.like(query.asset_mint.clone())
                                .and(currency_mint.like(currency_mint))
                                .and(timestamp.le(to)))
                        .limit(query.limit.unwrap_or(100))
                        .load::<Trade>(&mut db)
                        .expect("Error loading cursors")
                }
            }
        }
    };


    create_trade_response(&cursor_db)
}






