use std::{env, net::Ipv4Addr, sync::Arc};

use log::info;
use tokio::net::unix::SocketAddr;
use utoipa::{
    Modify,
    OpenApi, openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_swagger_ui::Config;
use warp::{
    Filter,
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail}, Rejection, Reply,
};
use warp::http::Method;
use warp::http::uri::Port;

use database_psql::connection::create_psql_pool_diesel;
//use types::trade_t;
use database_psql::model::Trade;
use endpoints::udf::udf_config_t;
use endpoints::udf::udf_history_t;
use endpoints::udf::udf_search_t;
use endpoints::udf::udf_symbol_info_t;
use endpoints::udf::udf_symbols_t;

use crate::endpoints::default::default;
use crate::endpoints::stats::stats;
use crate::endpoints::trades::trades;
use crate::endpoints::udf::udf;

mod endpoints;
mod helper;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Arc::new(Config::from("/api-doc.json"));

    #[derive(OpenApi)]
    #[openapi(
    paths(
    default::get_info,
    trades::get_base,
    trades::get_symbol,
    trades::get_signature,
    trades::get_address,
    trades::get_mint,
    trades::get_volume,
    udf::get_home,
    udf::get_time,
    udf::get_config,
    udf::get_symbol_info,
    udf::get_symbols,
    udf::get_search,
    udf::get_history,
    stats::get_last_timestamp,
    stats::get_first_timestamp,
    stats::get_ranges,
    ),
    components(
    schemas(
    trades::VolumeData,
    database_psql::model::Trade,
    database_psql::model::Cursor,
    udf_config_t::UdfConfig,
    udf_config_t::Exchange,
    udf_config_t::SymbolsType,
    udf_symbol_info_t::UdfSymbolInfo,
    udf_search_t::UdfSearchSymbol,
    udf_history_t::UdfHistory)
    ),
    modifiers(& SecurityAddon),
    tags(
    (name = "default", description = "Default Data endpoints"),
    (name = "udf", description = "UDF compatible endpoints"),
    (name = "stats", description = "Stats endpoints"),
    (name = "trades", description = "Trade endpoints")
    )
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }

    let root = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::with_status("Hello there!\nPlease visit: /docs", StatusCode::OK));

    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    let swagger_ui = warp::path("docs")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_swagger);

    let port = env::var("APIPORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap();

    println!("Running on http://{}:{}/docs/", Ipv4Addr::UNSPECIFIED, port);
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET]);

    warp::serve(
        root.or(api_doc)
            .or(swagger_ui)
            .or(default::handlers().await.with(cors.clone()))
            .or(udf::handlers().await.with(cors.clone()))
            .or(stats::handlers().await.with(cors.clone()))
            .or(trades::handlers().await.with(cors)),
    )
        .run((Ipv4Addr::UNSPECIFIED, port))
        .await
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/docs" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static("/docs/"))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
