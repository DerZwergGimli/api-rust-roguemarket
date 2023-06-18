use std::{
    convert::Infallible,
    env,
    sync::{Arc, Mutex},
};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, NaiveDateTime, Utc};
use database_psql::connection::create_psql_pool_diesel;
use database_psql::connection::create_psql_raw_pool;
use deadpool_postgres::GenericClient;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use log::{info, warn};
use postgres::{NoTls, Row};
use serde::{Deserialize, Serialize};
use staratlas::symbolstore::{BuilderSymbolStore, SymbolStore};
use types::databasetrade::DBTrade;
use types::m_ohclvt::M_OHCLVT;
use utoipa::{IntoParams, ToSchema};
use warp::{Filter, hyper::StatusCode, Reply};
use warp::sse::reply;

use udf::time_convert::{convert_udf_time_to_seconds, convert_udf_time_to_timestamp_minute};

use crate::endpoints::udf::{udf_config_t, udf_history_t, udf_symbols_t};
use crate::endpoints::udf::{udf_search_t, udf_symbol_info_t};
use crate::endpoints::udf::helper::ohlc_converter;
use crate::endpoints::udf::udf_error_t::{Status, UdfError};
use crate::helper::{with_psql_store, with_raw_psql_store};
use crate::udf_config_t::{Exchange, SymbolsType};

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
    #[param(style = Form, example = "FOODATLAS")]
    symbol: String,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SearchParams {
    #[param(style = Form, example = "SymbolsParams")]
    query: String,
    #[serde(rename = "type")]
    ship_type: Option<String>,
    exchange: Option<String>,
    #[param(style = Form, example = "2")]
    limit: usize,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct HistoryParams {
    #[param(style = Form, example = "FOODATLAS")]
    symbol: String,
    #[param(style = Form, example = "1677799981")]
    from: Option<i64>,
    #[param(style = Form, example = "1678663981")]
    to: Option<i64>,
    resolution: Option<String>,
    countback: Option<u64>,
    #[serde(rename = "currencyCode")]
    currency_code: Option<String>,
}
//endregion

//region HANDLERS
pub async fn handlers() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone
{
    let store_sa = BuilderSymbolStore::new().init().await;
    let psql_raw_pool = create_psql_raw_pool();
    let psql_pool = create_psql_pool_diesel();

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
        .and(with_raw_psql_store(psql_raw_pool.clone()))
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


// fn with_mongo_store(
//     store: Collection<Document>,
// ) -> impl Filter<Extract=(Collection<Document>, ), Error=Infallible> + Clone {
//     warp::any().map(move || store.clone())
// }
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

    if filtered.len() == 1 {
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
                .contains(query.ship_type.clone().unwrap_or("".to_string()).as_str())
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
    db_pool: deadpool_postgres::Pool,
    query: HistoryParams,
) -> Result<impl Reply, Infallible> {
    let mut history = udf_history_t::UdfHistory {
        s: "ok".to_string(),
        t: vec![],
        c: vec![],
        o: vec![],
        h: vec![],
        l: vec![],
        v: vec![],
    };

    let mut db = db_pool.get().await.expect("Unable to get connection from pool!");

    let candle_timeframe_seconds = convert_udf_time_to_timestamp_minute(query.resolution).unwrap_or(60) * 60;
    println!("candle_timeframe_seconds: {}", candle_timeframe_seconds);

    let data: Vec<Row> = match query.countback {
        None => {
            db.query(
                "SELECT
                            time_bucket_gapfill($4, timestamp) AS bucket,
                            first(price, timestamp) AS open,
                            max(price) AS high,
                            min(price) AS low,
                            last(price, timestamp) AS close,
                            sum(asset_change) AS volume
                        FROM trades
                        WHERE symbol like $1
                        AND timestamp >= $2 AND timestamp < $3
                        GROUP BY bucket
                        ORDER BY bucket ASC ;",
                &[&query.symbol, &query.from.unwrap_or_default(), &query.to.unwrap_or_default(), &candle_timeframe_seconds],
            ).await.unwrap_or_default()
        }
        Some(countback) => {
            let c = countback as i64;
            db.query(
                "SELECT
                            time_bucket_gapfill($4, timestamp) AS bucket,
                            first(price, timestamp) AS open,
                            max(price) AS high,
                            min(price) AS low,
                            last(price, timestamp) AS close,
                            sum(asset_change) AS volume
                        FROM trades
                        WHERE symbol like $1
                        AND timestamp >= $2 AND timestamp < $3
                        GROUP BY bucket
                        ORDER BY bucket ASC
                        LIMIT $5 ;",
                &[&query.symbol, &query.from.unwrap_or_default(), &query.to.unwrap_or_default(), &candle_timeframe_seconds, &c],
            ).await.expect("Error querrying using countback")
        }
    };


    data.into_iter().for_each(|d| {
        history.t.push(d.get("bucket"));
        history.c.push(d.try_get("close").unwrap_or(*history.clone().c.last().unwrap_or(&0.0)));
        history.o.push(d.try_get("open").unwrap_or(*history.clone().c.last().unwrap_or(&0.0)));
        history.h.push(d.try_get("high").unwrap_or(*history.clone().c.last().unwrap_or(&0.0)));
        history.l.push(d.try_get("low").unwrap_or(*history.clone().c.last().unwrap_or(&0.0)));
        history.v.push(d.try_get("volume").unwrap_or_default());
    });

    if history.c.clone().into_iter().all(|close| close == 0.0)
    {
        history.t = vec![]
    }


    return if history.t.is_empty() {
        let last_timestamp: Vec<Row> = db.query("SELECT timestamp
                    FROM trades
                    WHERE symbol like $1
                    AND timestamp <= $2
                    ORDER BY timestamp DESC
                    LIMIT 1", &[&query.symbol, &query.to.unwrap_or_default()]).await.unwrap_or_default();

        if !last_timestamp.is_empty() {
            let timestamp: i64 = last_timestamp[0].get("timestamp");

            return Ok(warp::reply::json(&UdfError {
                s: Status::no_data,
                nextTime: Some(timestamp * 1000),
            }));
        }
        warn!("There seems to be no data...");
        return Ok(warp::reply::json(&UdfError {
            s: Status::no_data,
            nextTime: None,
        }));
    } else {
        Ok(warp::reply::json(&history))
    };
}
