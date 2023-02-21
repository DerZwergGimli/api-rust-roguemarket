use crate::endpoints::udf::udf_error_t::{Status, UdfError};
use crate::endpoints::udf::{udf_config_t, udf_history_t, udf_symbols_t};
use crate::endpoints::udf::{udf_search_t, udf_symbol_info_t};
use crate::udf_config_t::{Exchange, SymbolsType};
use log::info;
use mongo::mongodb::{
    find_by_signature, find_by_symbol, find_udf_trade_next, find_udf_trades, MongoDBConnection,
};
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
    address: String,
}
//endregion

//region HANDLERS
pub async fn handlers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let mongo_db =
        MongoDBConnection::new(env::var("MONGOURL").expect("NO MONGOURL").as_str()).await;

    let home = warp::path!("info")
        .and(warp::get())
        .and(warp::path::end())
        .and_then(get_info);

    let info = warp::path!("last")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_mongo_store(
            mongo_db.collection_as_doc.clone(),
        ))
        .and(warp::query::<DefaultLastParams>())
        .and_then(get_last);

    let signature = warp::path!("signature")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_mongo_store(
            mongo_db.collection_as_doc.clone(),
        ))
        .and(warp::query::<DefaultSignatureParams>())
        .and_then(get_signature);

    home.or(info).or(signature)
}

fn with_mongo_store(
    store: Collection<Document>,
) -> impl Filter<Extract = (Collection<Document>,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
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

/// Get last trade
///
/// Responses with a last trade for a given symbol. [max. 100]
#[utoipa::path(
get,
path = "/last",
params(DefaultLastParams),
responses(
(status = 200, description = "Response: Time successful", body = String)
)
)]
pub async fn get_last(
    trades: Collection<Document>,
    query: DefaultLastParams,
) -> Result<impl Reply, Infallible> {
    match find_by_symbol(trades.clone(), query.symbol.clone(), query.limit.clone()).await {
        Some(data) => {
            return Ok(warp::reply::json(&data));
        }
        _ => {
            /// A placeholder for a future error handling.
            let error = "Error".to_string();
            return Ok(warp::reply::json(&error));
        }
    };
}

/// Get trade by signature
///
/// Responses with a trade for a given signature.
#[utoipa::path(
get,
path = "/signature",
params(DefaultSignatureParams),
responses(
(status = 200, description = "Response: Time successful", body = String)
)
)]
pub async fn get_signature(
    trades: Collection<Document>,
    query: DefaultSignatureParams,
) -> Result<impl Reply, Infallible> {
    match find_by_signature(trades.clone(), query.address.clone()).await {
        Some(data) => {
            return Ok(warp::reply::json(&data));
        }
        _ => {
            /// A placeholder for a future error handling.
            let error = "Error".to_string();
            return Ok(warp::reply::json(&error));
        }
    };
}
