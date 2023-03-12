extern crate core;

mod pb;
mod substreams;
mod substreams_stream;
mod mongodb;

use reqwest::header;
use std::{env, fs, thread};
use std::os::unix::raw::ino_t;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use ::mongodb::Database;
use anyhow::{Context, Error, format_err};

use json::object;
use log::{error, info, warn};
use prost::{DecodeError};
use tokio_stream::StreamExt;
use crate::pb::substreams::module_output::Data::MapOutput;
use crate::pb::substreams::{BlockScopedData, Request, StoreDeltas};
use crate::pb::substreams::module_output::Data;
use prost::Message;
use staratlas::symbolstore::{Asset, BuilderSymbolStore, SymbolStore};
use tokio::sync::Mutex;
use types::trade_t::SATrade;
use crate::mongodb::{database_connect, database_create, database_cursor_update, database_cursor_get, database_cursor_create};
use crate::pb::substreams::stream_client::StreamClient;
use crate::pb::substreams::Package;
use crate::substreams::SubstreamsEndpoint;
use crate::substreams_stream::{BlockResponse, SubstreamsStream};
use crate::pb::database::{DatabaseChanges, TableChange};
use crate::pb::database::table_change::Operation;
use tokio::task;
use async_scoped;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};


#[derive(Debug, Clone)]
struct AppData {
    mongo_url: String,
    endpoint_url: String,
    package_file: String,
    module_name: String,
    database_name: String,
    start_block: i64,
    stop_block: u64,
    thread_count: u64,
    database: Option<Database>,
    symbol_store: Option<SymbolStore>,
    package: Package,
    multi_pg: MultiProgress,
    progress_main: ProgressBar,
}


#[tokio::main]
async fn main_old() -> Result<(), Error> {
    env_logger::init();
    let mut app_data = AppData {
        mongo_url: env::args().nth(1).expect("please provide a <mongo_url>"),
        endpoint_url: env::args().nth(2).expect("please provide a <endpoint_url>"),
        package_file: env::args().nth(3).expect("please provide a <package_file>"),
        module_name: env::args().nth(4).expect("please provide a <module_name>"),
        database_name: env::args().nth(5).expect("please provide a <database>"),
        start_block: env::args().nth(6).expect("please provide a <start_block>").parse::<i64>().unwrap_or(0),
        stop_block: env::args().nth(7).expect("please provide a <stop_block>").parse::<u64>().unwrap_or(0),
        thread_count: env::args().nth(8).expect("may provide a <thread_count> - this is set to 0 else!").parse::<u64>().unwrap_or(0),
        database: None,
        symbol_store: None,
        package: Default::default(),
        multi_pg: Default::default(),
        progress_main: (ProgressBar::new(0)),
    };
    if app_data.thread_count == 0 {
        app_data.thread_count = 1;
    }

    //Setup Progress
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
        .unwrap()
        .progress_chars("##-");
    app_data.multi_pg = MultiProgress::new();
    app_data.progress_main = app_data.multi_pg.add(ProgressBar::new(app_data.thread_count));
    app_data.progress_main.set_style(sty.clone());
    app_data.progress_main.set_message("total  ");
    app_data.progress_main.tick();


    let mut token: Option<String> = request_token(env::var("STREAMINGFAST_KEY").expect("please set env with: STREAMINGFAST_KEY")).await;

    info!("> Staring!");
    info!("{:?}", app_data.clone());
    //Connect DATABASE
    app_data.database = Some((database_connect(app_data.clone().mongo_url.to_string()).await?.database(app_data.clone().database_name.as_str())));
    app_data.symbol_store = Some(BuilderSymbolStore::new().init().await);

    //Read provided SF config
    app_data.package = read_package(&app_data.clone().package_file)?;
    let endpoint = Arc::new(SubstreamsEndpoint::new(&app_data.clone().endpoint_url, token.clone()).await?);


    let app_data_arc = Arc::new((
        app_data.clone()
    ));

    //Spawn up Threads
    info!("...Starting in parallel work mode! With {:} Threads", app_data.clone().thread_count);
    std::panic::catch_unwind(|| {}).unwrap();

    let range: i64 = (app_data.clone().stop_block - app_data.clone().start_block as u64) as i64;
    if range < 0 {
        panic!("Please provide a valid block-range");
    }
    let range_block = range.abs() / app_data.clone().thread_count as i64;


    let mut threads = vec![];
    for t in 0..app_data.clone().thread_count {
        info!("Staring Thread_{:}", t);
        let app_data_ = Arc::clone(&app_data_arc);
        let endpoint_ = Arc::clone(&endpoint);
        let package = read_package(&app_data.clone().package_file)?;

        threads.push(tokio::spawn(async move {
            run_substream(
                t as usize,
                (app_data_.clone().start_block + range_block * t as i64) as u64,
                (app_data_.clone().start_block + range_block + range_block * t as i64) as u64,
                app_data_.clone(),
                app_data_.database.clone().unwrap(),
                app_data_.symbol_store.clone().unwrap(),
                package.clone(),
                endpoint_.clone()).await.expect("TODO: panic message");
        }
        ))
    };

    let results = futures::future::join_all(threads).await;
    let mut errors = Vec::new();

    for result in results {
        if let Err(error) = result {
            errors.push(error);
        }
    }

    // Handle any errors that occurred during execution
    if !errors.is_empty() {
        for error in errors {
            eprintln!("Error occurred: {:?}", error);
        }
    }

    // for t in threads {
    //     tokio::join!(t);
    // }


    Ok(())
}


async fn run_substream(task_index: usize,
                       start: u64,
                       stop: u64,
                       app_data: Arc<AppData>,
                       database: Database,
                       symbol_store: SymbolStore,
                       package: Package,
                       endpoint: Arc<SubstreamsEndpoint>) -> Result<(), Error> {
    let cursor_name = app_data.module_name.clone().to_owned() + app_data.start_block.clone().to_string().as_str();
    let cursor = database_cursor_get(database.clone(), cursor_name.clone()).await;
    info!("cursor={:?}", cursor.clone());
    database_cursor_create(database.clone(), cursor_name.clone(), cursor.clone()).await?;


    let mut stream = SubstreamsStream::new(
        endpoint.clone(),
        cursor.clone(),
        package.modules.clone(),
        app_data.module_name.clone().to_string(),
        start as i64,
        stop,
    );


    let progress_bar = app_data.multi_pg.insert_before(&app_data.progress_main, ProgressBar::new(stop - start));
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
        .unwrap()
        .progress_chars("##-");

    progress_bar.set_style(sty.clone());

    loop {
        progress_bar.set_message(format!("Task #{}", task_index));
        match stream.next().await {
            None => {
                //println!("Stream consumed");
                sleep(Duration::from_secs(1));
                progress_bar.finish();
                app_data.progress_main.inc(1);
                break;
            }
            Some(event) => match event {
                Err(_) => {
                    println!("Error");
                    panic!("Error while handling stream?");
                    //progress_bar.inc(1);
                }
                Ok(BlockResponse::New(data)) => {
                    let cursor = Some(data.cursor.clone());
                    match extract_database_changes_from_map(data.clone(), &app_data.module_name.to_owned()) {
                        Ok(DatabaseChanges { table_changes }) => {
                            for table_changed in table_changes {
                                match table_changed.operation() {
                                    Operation::Unset => {
                                        warn!("operation not supported")
                                    }
                                    Operation::Create => {
                                        let mapped = map_trade_to_struct(table_changed, symbol_store.clone())?;
                                        database_create(database.clone(), mapped, "trades".to_string()).await?;
                                    }
                                    Operation::Update => {
                                        warn!("operation not supported")
                                    }
                                    Operation::Delete => {
                                        warn!("operation not supported")
                                    }
                                }
                            }
                            //Update cursor
                            database_cursor_update(database.clone(), cursor_name.clone(), cursor.clone()).await?;
                        }
                        Err(error) => {
                            error!("not correct module");
                        }
                    }
                    progress_bar.inc(data.step as u64);
                }
            },
        }
    }


    sleep(Duration::from_millis(env::args().nth(8).unwrap_or("0".to_string()).parse::<u64>().unwrap()));
    Ok(())
}


async fn request_token(key: String) -> Option<String> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());

    let client = reqwest::Client::new();
    let res = client.post("https://auth.streamingfast.io/v1/auth/issue")
        .headers(headers)
        .body(object! {"api_key": key}.to_string())
        .send()
        .await.unwrap()
        .text().await.unwrap();

    let json_response = json::parse(res.clone().as_str()).expect("Error parsing response!");


    let d = json_response["token"].to_string();
    println!("res={:?}", res.clone());

    Some(d.clone())
}

pub fn decode<T: std::default::Default + prost::Message>(buf: &Vec<u8>) -> Result<T, DecodeError> {
    ::prost::Message::decode(&buf[..])
}

fn read_package(file: &str) -> Result<Package, anyhow::Error> {
    use prost::Message;
    let content = std::fs::read(file).context(format_err!("read package {}", file))?;
    Package::decode(content.as_ref()).context("decode command")
}

fn extract_database_changes_from_map(data: BlockScopedData, module_name: &String) -> Result<DatabaseChanges, Error> {
    let output = data
        .outputs
        .first()
        .ok_or(format_err!("expecting one module output"))?;
    if &output.name != module_name {
        return Err(format_err!(
            "invalid module output name {}, expecting {}",
            output.name,
            module_name
        ));
    }

    match output.data.as_ref().unwrap() {
        MapOutput(data) => {
            let wrapper: DatabaseChanges = Message::decode(data.value.as_slice())?;
            Ok(wrapper)
        }
        _ => {
            Err(format_err!("invalid module output StoreDeltas, expecting MapOutput"))
        }
    }
}

fn map_trade_to_struct(table_change: TableChange, symbol_store: SymbolStore) -> Result<SATrade, Error> {
    let mut trade = SATrade {
        symbol: "-none-".to_string(),
        signature: table_change.clone().pk,
        block: table_change.clone().fields.into_iter().find(|t| { t.name.contains("block") }).unwrap().new_value.parse().unwrap_or(0),
        timestamp: table_change.clone().fields.into_iter().find(|t| { t.name == "timestamp" }).ok_or("timestamp").unwrap().new_value.parse().unwrap_or(0),
        order_taker: table_change.clone().fields.into_iter().find(|t| { t.name == "order_taker" }).ok_or("order_taker").unwrap().new_value,
        currency_mint: table_change.clone().fields.into_iter().find(|t| { t.name == "currency_mint" }).ok_or("currency_mint").unwrap().new_value,
        asset_mint: table_change.clone().fields.into_iter().find(|t| { t.name == "asset_mint" }).ok_or("asset_mint").unwrap().new_value,
        order_initializer: table_change.clone().fields.into_iter().find(|t| { t.name == "order_initializer" }).ok_or("order_initializer").unwrap().new_value,
        asset_change: table_change.clone().fields.into_iter().find(|t| { t.name == "asset_change" }).ok_or("asset_change").unwrap().new_value.parse().unwrap_or(0.0),
        market_fee: table_change.clone().fields.into_iter().find(|t| { t.name == "market_fee" }).ok_or("market_fee").unwrap().new_value.parse().unwrap_or(0.0),
        total_cost: table_change.clone().fields.into_iter().find(|t| { t.name == "total_cost" }).ok_or("total_cost").unwrap().new_value.parse().unwrap_or(0.0),
    };

    match symbol_store.assets.into_iter().find(|asset| { asset.mint == trade.asset_mint && asset.pair_mint == trade.currency_mint }) {
        None => {}
        Some(symbol) => { trade.symbol = symbol.symbol }
    }

    return Ok(trade);
}
