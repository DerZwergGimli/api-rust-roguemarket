use std::{
    convert::Infallible,
    env,
    sync::{Arc, Mutex},
};
use std::future::Future;
use std::time::{SystemTime, UNIX_EPOCH};

use log::info;
use serde::{Deserialize, Serialize};
use staratlas::symbolstore::{BuilderSymbolStore, SymbolStore};
use types::databasetrade::DBTrade;
use types::m_ohclvt::M_OHCLVT;
use utoipa::{IntoParams, ToSchema};
use warp::{Filter, hyper::StatusCode, Reply};
use warp::sse::reply;

use database_psql::connection::create_psql_pool_diesel;

use crate::endpoints::udf::{udf_config_t, udf_history_t, udf_symbols_t};
use crate::endpoints::udf::{udf_search_t, udf_symbol_info_t};
use crate::endpoints::udf::udf_error_t::{Status, UdfError};
use crate::udf_config_t::{Exchange, SymbolsType};

//region PARAMS

//endregion

//region HANDLERS
pub async fn handlers() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone
{
    let psql_pool = create_psql_pool_diesel();
    let home = warp::path!("info")
        .and(warp::get())
        .and(warp::path::end())
        .and_then(get_info);


    home
}
//endregion

/// Info
///
/// Responses with info object.
#[utoipa::path(
get,
path = "/info",
responses(
(status = 200, description = "Get time successfully", body = String)
)
)]
pub async fn get_info() -> Result<impl Reply, Infallible> {
    let message = "Hello from RogueMarket API".to_string();
    Ok(warp::reply::with_status(message, StatusCode::OK))
}

