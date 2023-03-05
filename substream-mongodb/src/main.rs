extern crate core;

mod pb;
mod substreams;
mod substreams_stream;
mod mongodb;


use reqwest::header;

use std::{env, fs};
use std::os::unix::raw::ino_t;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
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
use types::trade_t::SATrade;
use crate::mongodb::{database_connect, database_create, database_cursor_update, database_cursor_get, database_cursor_create};
use crate::pb::substreams::stream_client::StreamClient;
use crate::pb::substreams::Package;
use crate::substreams::SubstreamsEndpoint;
use crate::substreams_stream::{BlockResponse, SubstreamsStream};
use crate::pb::database::{DatabaseChanges, TableChange};
use crate::pb::database::table_change::Operation;


#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let mongo_url = env::args().nth(1).expect("please provide a <mongo_url>");
    let endpoint_url = env::args().nth(2).expect("please provide a <endpoint_url>");
    let package_file = env::args().nth(3).expect("please provide a <package_file>");
    let module_name = env::args().nth(4).expect("please provide a <module_name>");
    let database_name = env::args().nth(5).expect("please provide a <database>");
    let start_block = env::args().nth(6).expect("please provide a <start_block>").parse::<i64>().unwrap_or(179432144);
    let stop_block = env::args().nth(7).expect("please provide a <stop_block>").parse::<u64>().unwrap_or(179432145);


    let mut token: Option<String> = request_token(env::var("STREAMINGFAST_KEY").expect("please set env with: STREAMINGFAST_KEY")).await;

    info!("> Staring!");
    info!("mongo_url={:?}\nendpoint_url={:?}\npackage_file{:?}\nmodule_name={:?}\nstart-block={:}\nstop-block={:}", mongo_url.clone(), endpoint_url, &package_file, &module_name, start_block, stop_block);

    let database = database_connect(mongo_url).await?.database(database_name.as_str());
    let symbol_store = BuilderSymbolStore::new().init().await;


    let package = read_package(&package_file)?;
    let endpoint = Arc::new(SubstreamsEndpoint::new(&endpoint_url, token.clone()).await?);


    let cursor = database_cursor_get(database.clone(), module_name.clone()).await;
    info!("cursor={:?}", cursor.clone());
    database_cursor_create(database.clone(), module_name.clone().to_string(), cursor.clone()).await?;


    let mut stream = SubstreamsStream::new(
        endpoint.clone(),
        cursor.clone(),
        package.modules.clone(),
        module_name.clone().to_string(),
        start_block,
        stop_block,
    );

    info!("> Setup completed!");

    loop {
        match stream.next().await {
            None => {
                println!("Stream consumed");
                break;
            }

            Some(event) => match event {
                Err(_) => {
                    println!("Error");
                }
                Ok(BlockResponse::New(data)) => {
                    let cursor = Some(data.cursor.clone());
                    match extract_database_changes_from_map(data, &module_name) {
                        Ok(DatabaseChanges { table_changes }) => {
                            for table_changed in table_changes {
                                match table_changed.operation() {
                                    Operation::Unset => {
                                        warn!("operation not supported")
                                    }
                                    Operation::Create => {
                                        let mapped = map_trade_to_struct(table_changed, symbol_store.clone())?;
                                        database_create(database.clone(), mapped, "trades".to_string()).await?
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
                            database_cursor_update(database.clone(), module_name.clone().to_string(), cursor.clone()).await?;
                        }
                        Err(error) => {
                            error!("not correct module");
                        }
                    }
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

    Some(d)
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
