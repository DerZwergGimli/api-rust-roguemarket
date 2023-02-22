use crate::endpoints::udf::udf_error_t::{Status, UdfError};
use crate::endpoints::udf::{udf_config_t, udf_history_t, udf_symbols_t};
use crate::endpoints::udf::{udf_search_t, udf_symbol_info_t};
use crate::udf_config_t::{Exchange, SymbolsType};
use log::info;
use mongo::mongodb::{find_udf_trade_next, find_udf_trades, MongoDBConnection};
use mongodb::bson::Document;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use staratlas::symbolstore::{BuilderSymbolStore, SymbolStore};
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
pub struct SymbolInfoParams {
    #[param(style = Form, example = "FOOD")]
    group: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SymbolsParams {
    #[param(style = Form, example = "FOOD")]
    symbol: String,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SearchParams {
    #[param(style = Form, example = "FOOD")]
    query: String,
    #[serde(rename = "type")]
    shipType: Option<String>,
    exchange: Option<String>,
    #[param(style = Form, example = "2")]
    limit: usize,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct HistoryParams {
    #[param(style = Form, example = "FOOD")]
    symbol: String,
    from: Option<u64>,
    to: Option<u64>,
    resolution: Option<String>,
    countback: Option<u64>,
}
//endregion

//region HANDLERS
pub async fn handlers() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone
{
    let store_sa = BuilderSymbolStore::new().init().await;
    let mongo_db =
        MongoDBConnection::new(env::var("MONGOURL").expect("NO MONGOURL").as_str()).await;

    let home = warp::path!("udf")
        .and(warp::get())
        .and(warp::path::end())
        .and_then(get_home);

    let time = warp::path!("udf" / "time")
        .and(warp::get())
        .and(warp::path::end())
        .and_then(get_time);

    let config = warp::path!("udf" / "config")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_sa_store(store_sa.clone()))
        .and_then(get_config);

    let symbol_info = warp::path!("udf" / "symbol_info")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_sa_store(store_sa.clone()))
        .and_then(get_symbol_info);

    let symbols = warp::path!("udf" / "symbols")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_sa_store(store_sa.clone()))
        .and(warp::query::<SymbolsParams>())
        .and_then(get_symbols);

    let search = warp::path!("udf" / "search")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_sa_store(store_sa.clone()))
        .and(warp::query::<SearchParams>())
        .and_then(get_search);

    let history = warp::path!("udf" / "history")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_mongo_store(
            mongo_db.collection.clone(),
        ))
        .and(warp::query::<HistoryParams>())
        .and_then(get_history);

    home.or(config)
        .or(time)
        .or(symbol_info)
        .or(symbols)
        .or(search)
        .or(history)
}

fn with_sa_store(
    store: SymbolStore,
) -> impl Filter<Extract=(SymbolStore, ), Error=Infallible> + Clone {
    warp::any().map(move || store.clone())
}

fn with_mongo_store(
    store: Collection<DBTrade>,
) -> impl Filter<Extract=(Collection<DBTrade>, ), Error=Infallible> + Clone {
    warp::any().map(move || store.clone())
}
//endregion

/// UDF Home
///
/// Responses with server time.
#[utoipa::path(
get,
path = "/udf/",
responses(
(status = 200, description = "Get time successfully", body = String)
)
)]
pub async fn get_home() -> Result<impl Reply, Infallible> {
    let message = "Hello this is a UDF compatible route!".to_string();
    Ok(warp::reply::with_status(message, StatusCode::OK))
}

/// Get ServerTime
///
/// Responses with server time.
#[utoipa::path(
get,
path = "/udf/time",
responses(
(status = 200, description = "Response: Time successful", body = String)
)
)]
pub async fn get_time() -> Result<impl Reply, Infallible> {
    let time = SystemTime::now().duration_since(UNIX_EPOCH);
    let time_string = time.unwrap_or_default().as_secs();

    Ok(warp::reply::with_status(
        time_string.to_string(),
        StatusCode::OK,
    ))
}

/// Get UDF-Config
///
/// Responses with a UDF config in json.
#[utoipa::path(
get,
path = "/udf/config",
responses(
(status = 200, description = "Response: Config successful", body = [UdfConfig])
)
)]
pub async fn get_config(store: SymbolStore) -> Result<impl Reply, Infallible> {
    let mut all_symbols = Vec::new();

    store
        .exchange
        .clone()
        .asset_type
        .into_iter()
        .for_each(|asset_type| {
            all_symbols.push(SymbolsType {
                value: asset_type.to_string(),
                name: asset_type.to_string(),
            })
        });

    let config = udf_config_t::UdfConfig {
        exchanges: vec![Exchange {
            value: store.exchange.clone().symbol,
            name: store.exchange.clone().name,
            desc: store.exchange.clone().description,
        }],

        symbols_types: all_symbols,
        supported_resolutions: store.exchange.clone().supported_resolutions,
        supports_search: store.exchange.clone().supports_search,
        supports_group_request: store.exchange.clone().supports_group_request,
        supports_marks: store.exchange.clone().supports_marks,
        supports_timescale_marks: store.exchange.clone().supports_timescale_marks,
        supports_time: store.exchange.clone().supports_time,
    };
    Ok(warp::reply::json(&config))
}

/// Get Symbol group request
///
/// Responses with a SymbolGroup in json.
#[utoipa::path(
get,
path = "/udf/symbol_info",
params(SymbolInfoParams),
responses(
(status = 200, description = "Response: SymbolInfo successful", body = [UdfSymbolInfo])
)
)]
pub async fn get_symbol_info(store: SymbolStore) -> Result<impl Reply, Infallible> {
    let config = udf_symbol_info_t::UdfSymbolInfo {
        symbol: store
            .assets
            .clone()
            .into_iter()
            .map(|asset| asset.symbol)
            .collect(),
        ticker: store
            .assets
            .clone()
            .into_iter()
            .map(|asset| asset.symbol)
            .collect(),
        name: store
            .assets
            .clone()
            .into_iter()
            .map(|asset| asset.symbol)
            .collect(),
        full_name: store
            .assets
            .clone()
            .into_iter()
            .map(|asset| asset.symbol)
            .collect(),
        description: store
            .assets
            .clone()
            .into_iter()
            .map(|asset| asset.description)
            .collect(),
        exchange: store.exchange.clone().name,
        listed_exchange: store.exchange.clone().name,
        udf_symbol_info_type: store.exchange.clone().asset_type[0].clone(),
        currency_code: store
            .assets
            .clone()
            .into_iter()
            .map(|asset| asset.pair_name)
            .collect(),
        session: store.exchange.clone().sesstion,
        timezone: store.exchange.clone().timezone,
        minmovement: store.exchange.clone().minmovement,
        minmov: store.exchange.clone().minmov,
        minmovement2: store.exchange.clone().minmovement2,
        minmov2: store.exchange.clone().minmov2,
        pricescale: store
            .assets
            .clone()
            .into_iter()
            .map(|asset| asset.pricescale)
            .collect(),
        supported_resolutions: store.exchange.clone().supported_resolutions,
        has_intraday: store.exchange.clone().has_intraday,
        has_daily: store.exchange.clone().has_daily,
        has_weekly_and_monthly: store.exchange.clone().has_weekly_and_monthly,
        data_status: store.exchange.data_status,
    };

    Ok(warp::reply::json(&config))
}

/// Get Symbol resolve request
///
/// Responses with a SymbolInfo in json.
#[utoipa::path(
get,
path = "/udf/symbols",
params(SymbolsParams),
responses(
(status = 200, description = "Response: SymbolInfo successful", body = [UdfSymbolInfo]),
(status = 404, description = "Nothing found")
)
)]
pub async fn get_symbols(
    store: SymbolStore,
    query: SymbolsParams,
) -> Result<Box<dyn Reply>, Infallible> {
    let filtered = store
        .assets
        .clone()
        .into_iter()
        .filter(|asset| asset.symbol == query.symbol)
        .collect::<Vec<_>>();

    if (filtered.len() == 1) {
        let symbols = udf_symbols_t::UdfSymbols {
            symbol: filtered[0].clone().symbol,
            ticker: filtered[0].clone().symbol,
            name: filtered[0].clone().symbol,
            full_name: filtered[0].clone().symbol,
            description: filtered[0].clone().description,
            exchange: store.exchange.clone().name,
            listed_exchange: store.exchange.clone().name,
            udf_symbols_type: filtered[0].clone().asset_type,
            currency_code: filtered[0].clone().pair_name,
            session: store.exchange.clone().sesstion,
            timezone: store.exchange.clone().timezone,
            minmovement: store.exchange.clone().minmovement,
            minmov: store.exchange.clone().minmov,
            minmovement2: store.exchange.clone().minmovement2,
            minmov2: store.exchange.clone().minmov2,
            pricescale: filtered[0].clone().pricescale,
            supported_resolutions: store.exchange.clone().supported_resolutions,
            has_intraday: store.exchange.clone().has_intraday,
            has_daily: store.exchange.clone().has_daily,
            has_weekly_and_monthly: store.exchange.clone().has_weekly_and_monthly,
            data_status: store.exchange.data_status,
        };
        return Ok(Box::new(warp::reply::json(&symbols)));
    }
    return Ok(Box::new(StatusCode::NOT_FOUND));
}

/// Get Search request
///
/// Responses with a Search-Result in json.
#[utoipa::path(
get,
path = "/udf/search",
params(SearchParams),
responses(
(status = 200, description = "Response: SymbolInfo successful", body = [UdfSearchSymbol])
)
)]
pub async fn get_search(store: SymbolStore, query: SearchParams) -> Result<impl Reply, Infallible> {
    let filtered = store
        .assets
        .into_iter()
        .filter(|asset| {
            asset.symbol.contains(query.query.clone().as_str())
                && asset
                .asset_type
                .contains(query.shipType.clone().unwrap_or("".to_string()).as_str())
        })
        .collect::<Vec<_>>();

    let mut search: Vec<udf_search_t::UdfSearchSymbol> = Vec::new();
    filtered.into_iter().for_each(|asset| {
        search.push(udf_search_t::UdfSearchSymbol {
            symbol: asset.clone().symbol,
            full_name: asset.clone().asset_name,
            description: asset.clone().description,
            exchange: store.exchange.clone().symbol,
            ticker: asset.clone().symbol,
            udf_symbol_type: asset.clone().asset_type,
        })
    });

    let mut search_limited: Vec<udf_search_t::UdfSearchSymbol> = Vec::new();
    if (query.limit > 0) {
        if (search.len() < query.limit) {
            for l in 0..search.len() {
                search_limited.push(search[l].clone());
            }
        } else {
            for l in 0..query.limit {
                search_limited.push(search[l].clone());
            }
        }
    }

    Ok(warp::reply::json(&search_limited))
}

/// Get History request
///
/// Responses with a History-Result in json (oclh).
#[utoipa::path(
get,
path = "/udf/history",
params(HistoryParams),
responses(
(status = 200, description = "Response: SymbolInfo successful", body = [UdfHistory])
)
)]
pub async fn get_history(
    trades: Collection<DBTrade>,
    query: HistoryParams,
) -> Result<impl Reply, Infallible> {
    let mut history = udf_history_t::UdfHistory {
        s: "".to_string(),
        t: vec![],
        c: vec![],
        o: vec![],
        h: vec![],
        l: vec![],
        v: vec![],
    };

    match find_udf_trades(
        trades.clone(),
        query.symbol.clone(),
        query.from.unwrap_or_default(),
        query.to.unwrap_or_default(),
        convert_udf_time_to_minute(query.resolution.unwrap_or("3600".to_string()).as_str())
            .unwrap(),
        query.countback,
    )
        .await
    {
        Some(data) => {
            if (data.len() > 0) {
                history.s = "ok".to_string();
                history.o = data.clone().into_iter().map(|d| d.open).collect();
                history.h = data.clone().into_iter().map(|d| d.high).collect();
                history.c = data.clone().into_iter().map(|d| d.close).collect();
                history.l = data.clone().into_iter().map(|d| d.low).collect();
                history.v = data.clone().into_iter().map(|d| d.volume).collect();
                history.t = data.clone().into_iter().map(|d| d.time_last).collect();
            }
        }
        _ => {}
    };

    if (history.t.len() > 0) {
        info!("found");
        return Ok(warp::reply::json(&history));
    } else {
        return match find_udf_trade_next(trades, query.symbol, query.to.unwrap_or_default()).await {
            Some(data) => {
                info!("no-data");
                return Ok(warp::reply::json(&UdfError {
                    s: Status::no_data,
                    errmsg: "No data found".to_string(),
                    nextTime: Some(data.timestamp),
                }));
            }
            None => {
                info!("error");
                return Ok(warp::reply::json(&UdfError {
                    s: Status::no_data,
                    errmsg: "No data found".to_string(),
                    nextTime: None,
                }));
            }
        };
    }
}
