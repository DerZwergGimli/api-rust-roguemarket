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
use utoipa::openapi::SchemaFormat::DateTime;
use warp::{Filter, hyper::StatusCode, Reply};
use warp::sse::reply;

use database_psql::connection::create_psql_pool;

use crate::endpoints::stats::stats_error::StatsError;
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
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultSignatureParams {
    signature: String,
    limit: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultAddressParams {
    address: String,
    limit: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DefaultMintParams {
    mint: String,
    limit: Option<i64>,
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
/// Responses with a last trade for a given symbol. [max. 100]
#[utoipa::path(
get,
path = "/trades/last",
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

    let cursor_db: Vec<Trade> = trades
        .filter(symbol.like(query.symbol.clone()))
        .limit(query.limit.unwrap_or(10))
        .load::<Trade>(&mut db)
        .expect("Error loading cursors");

    return if cursor_db.is_empty() {
        warn!("There seems to be no data...");
        Ok(warp::reply::json(&StatsError {
            s: 1,
            errmsg: "No data found".to_string(),
        }))
    } else {
        Ok(warp::reply::json(&cursor_db))
    };
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

    let cursor_db: Vec<Trade> = trades
        .filter(signature.like(query.signature.clone()))
        .limit(query.limit.unwrap_or(10))
        .load::<Trade>(&mut db)
        .expect("Error loading cursors");

    return if cursor_db.is_empty() {
        warn!("There seems to be no data...");
        Ok(warp::reply::json(&StatsError {
            s: 1,
            errmsg: "No data found".to_string(),
        }))
    } else {
        Ok(warp::reply::json(&cursor_db))
    };
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

    let cursor_db: Vec<Trade> = trades
        .filter(order_taker.like(query.address.clone()).or(order_initializer.like(query.address)))
        .limit(query.limit.unwrap_or(100))
        .load::<Trade>(&mut db)
        .expect("Error loading cursors");

    return if cursor_db.is_empty() {
        warn!("There seems to be no data...");
        Ok(warp::reply::json(&StatsError {
            s: 1,
            errmsg: "No data found".to_string(),
        }))
    } else {
        Ok(warp::reply::json(&cursor_db))
    };
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

    let cursor_db: Vec<Trade> = trades
        .filter(
            asset_mint.like(query.mint.clone())
                .or(currency_mint.like(query.mint)))
        .limit(query.limit.unwrap_or(100))
        .load::<Trade>(&mut db)
        .expect("Error loading cursors");

    return if cursor_db.is_empty() {
        warn!("There seems to be no data...");
        Ok(warp::reply::json(&StatsError {
            s: 1,
            errmsg: "No data found".to_string(),
        }))
    } else {
        Ok(warp::reply::json(&cursor_db))
    };
}





