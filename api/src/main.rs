mod endpoints;
use crate::endpoints::udf::udf;
use endpoints::udf::udf_config_t;
use endpoints::udf::udf_history_t;
use endpoints::udf::udf_search_t;
use endpoints::udf::udf_symbolInfo_t;
use log::info;
use std::{env, net::Ipv4Addr, sync::Arc};
use tokio::net::unix::SocketAddr;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::Config;
use warp::http::uri::Port;
use warp::http::Method;
use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Filter, Rejection, Reply,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Arc::new(Config::from("/api-doc.json"));

    #[derive(OpenApi)]
    #[openapi(
    paths(
    udf::get_home,
    udf::get_time,
    udf::get_config,
    udf::get_symbol_info,
    udf::get_symbols,
    udf::get_search,
    udf::get_history,
    ),
    components(
    schemas(udf_config_t::UdfConfig, udf_symbolInfo_t::UdfSymbolInfo,
    udf_search_t::UdfSearchSymbol, udf_history_t::UdfHistory)
    ),
    modifiers(&SecurityAddon),
    tags(
    (name = "udf", description = "UDF compatible endpoints")
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

    warp::serve(api_doc.or(swagger_ui).or(udf::handlers().await.with(cors)))
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
