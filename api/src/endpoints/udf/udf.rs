use crate::endpoints::udf::{udf_config_t, udf_history_t, udf_symbols_t};
use crate::endpoints::udf::{udf_search_t, udf_symbolInfo_t};
use crate::udf_config_t::{Exchange, SymbolsType};
use serde::{Deserialize, Serialize};
use staratlas::symbolstore::{BuilderSymbolStore, SymbolStore};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use utoipa::openapi::SchemaFormat::DateTime;
use utoipa::{IntoParams, ToSchema};
use warp::sse::reply;
use warp::{hyper::StatusCode, Filter, Reply};

pub type Store = Arc<Mutex<Vec<UDF>>>;

/// Item to complete.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UDF {
    /// Unique database id.
    #[schema(example = 1)]
    id: i64,
    /// Description of what need to be done.
    #[schema(example = "Buy movie tickets")]
    value: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Order {
    AscendingId,
    DescendingId,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListQueryParams {
    /// Filters the returned `Todo` items according to whether they contain the specified string.
    #[param(style = Form, example = json!("task"))]
    contains: Option<String>,
    /// Order the returned `Todo` items.
    #[param(inline)]
    order: Option<Order>,
}

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
    symbol: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SearchParams {
    #[param(style = Form, example = "FOOD")]
    query: Option<String>,
    #[param(rename = "type")]
    stype: Option<String>,
    exchange: Option<String>,
    limit: Option<i32>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct HistoryParams {
    #[param(style = Form, example = "FOOD")]
    symbol: Option<String>,
    from: Option<u64>,
    to: Option<u64>,
    resolution: Option<String>,
    countback: Option<u64>,
}

pub async fn handlers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let store = Store::default();
    let storeSA = BuilderSymbolStore::new().init().await;

    let home = warp::path!("udf")
        .and(warp::get())
        .and(warp::path::end())
        .and_then(get_home);

    let time = warp::path!("udf" / "time")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_store(store.clone()))
        .and(warp::query::<ListQueryParams>())
        .and_then(get_time);

    let config = warp::path!("udf" / "config")
        .and(warp::get())
        .and(warp::path::end())
        .and(warp::query::<ListQueryParams>())
        .and_then(get_config);

    let symbol_info = warp::path!("udf" / "symbol_info")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_sa_store(storeSA.clone()))
        .and(warp::query::<ListQueryParams>())
        .and_then(get_symbol_info);

    let symbols = warp::path!("udf" / "symbols")
        .and(warp::get())
        .and(warp::path::end())
        .and(warp::query::<SymbolsParams>())
        .and_then(get_symbols);

    let search = warp::path!("udf" / "search")
        .and(warp::get())
        .and(warp::path::end())
        .and(warp::query::<SearchParams>())
        .and_then(get_search);

    let history = warp::path!("udf" / "history")
        .and(warp::get())
        .and(warp::path::end())
        .and(warp::query::<HistoryParams>())
        .and_then(get_history);

    home.or(config)
        .or(time)
        .or(symbol_info)
        .or(symbols)
        .or(search)
        .or(history)
}

fn with_store(store: Store) -> impl Filter<Extract = (Store,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
}
fn with_sa_store(
    store: SymbolStore,
) -> impl Filter<Extract = (SymbolStore,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
}

/// UDF Home
///
/// Responses with server time.
#[utoipa::path(
get,
path = "/udf/",
responses(
(status = 200, description = "Get time successfully", body = [Todo])
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
pub async fn get_time(store: Store, query: ListQueryParams) -> Result<impl Reply, Infallible> {
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
pub async fn get_config(query: ListQueryParams) -> Result<impl Reply, Infallible> {
    let config = udf_config_t::UdfConfig {
        exchanges: vec![Exchange {
            value: "GM".to_string(),
            name: "GalacticMarket".to_string(),
            desc: "StarAtlas GalacticMarket".to_string(),
        }],
        symbols_types: vec![SymbolsType {
            value: "nfts".to_string(),
            name: "StarAtlas Assets".to_string(),
        }],
        supported_resolutions: vec![
            "1".to_string(),
            "3".to_string(),
            "5".to_string(),
            "15".to_string(),
            "30".to_string(),
            "60".to_string(),
            "120".to_string(),
            "240".to_string(),
            "360".to_string(),
            "480".to_string(),
            "720".to_string(),
            "1D".to_string(),
            "3D".to_string(),
            "1W".to_string(),
            "1M".to_string(),
        ],
        supports_search: true,
        supports_group_request: false,
        supports_marks: false,
        supports_timescale_marks: false,
        supports_time: true,
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
pub async fn get_symbol_info(
    store: SymbolStore,
    query: ListQueryParams,
) -> Result<impl Reply, Infallible> {
    let config = udf_symbolInfo_t::UdfSymbolInfo {
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
        udf_symbol_info_type: store.exchange.clone().asset_type,
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
        has_intraday: true,
        has_daily: true,
        has_weekly_and_monthly: true,
        data_status: "streaming".to_string(),
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
(status = 200, description = "Response: SymbolInfo successful", body = [UdfSymbolInfo])
)
)]
pub async fn get_symbols(query: SymbolsParams) -> Result<impl Reply, Infallible> {
    let config = udf_symbols_t::UdfSymbols {
        symbol: "".to_string(),
        ticker: "".to_string(),
        name: "".to_string(),
        full_name: "".to_string(),
        description: "".to_string(),
        exchange: "".to_string(),
        listed_exchange: "".to_string(),
        udf_symbols_type: "".to_string(),
        currency_code: "".to_string(),
        session: "".to_string(),
        timezone: "".to_string(),
        minmovement: 0,
        minmov: 0,
        minmovement2: 0,
        minmov2: 0,
        pricescale: 0,
        supported_resolutions: vec![],
        has_intraday: false,
        has_daily: false,
        has_weekly_and_monthly: false,
        data_status: "".to_string(),
    };

    Ok(warp::reply::json(&config))
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
pub async fn get_search(query: SearchParams) -> Result<impl Reply, Infallible> {
    //TODO: make this a VEC

    let search = udf_search_t::UdfSearchSymbol {
        symbol: "".to_string(),
        full_name: "".to_string(),
        description: "".to_string(),
        exchange: "".to_string(),
        ticker: "".to_string(),
        udf_symbol_type: "".to_string(),
    };

    Ok(warp::reply::json(&search))
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
pub async fn get_history(query: HistoryParams) -> Result<impl Reply, Infallible> {
    //TODO: make this a VEC

    let search = udf_history_t::UdfHistory {
        s: "".to_string(),
        t: vec![],
        c: vec![],
        o: vec![],
        h: vec![],
        l: vec![],
        v: vec![],
    };

    Ok(warp::reply::json(&search))
}
