use std::sync::Arc;

use anyhow::{Error, format_err};
use chrono::NaiveDateTime;
use database_psql::model::Trade;
use database_psql::schema::trades::{asset_mint, symbol};
use indicatif::ProgressBar;
use json::object;
use log::info;
use reqwest::header;

use staratlas_symbols::symbol_store::SymbolStore;

use crate::pb::database::{DatabaseChanges, TableChange};
use crate::pb::pb_sa_trade::{ProcessExchange, ProcessExchanges};
use crate::pb::sf::substreams::rpc::v2::BlockScopedData;

#[derive(Debug)]
pub enum TaskStates {
    CREATING,
    INITIALIZING,
    STREAM_CONSUMED,
    RUNNING,
    INSERTING_DB,
    DONE,
}

pub fn update_task_info(pb_task: ProgressBar, task_index: usize, task_state: TaskStates) {
    pb_task.set_message(format!("Task_{}: {:?}", task_index, task_state));
}

pub async fn request_token(key: String) -> Option<String> {
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
    info!("token_request_info={:?}", res.clone());

    Some(d.clone())
}

pub fn extract_pb_sa_trades_from_map(data: BlockScopedData) -> Result<ProcessExchanges, Error> {
    let output = data.output.as_ref().unwrap().map_output.as_ref().unwrap();

    use prost::Message;
    let data: ProcessExchanges = Message::decode(output.value.as_slice()).unwrap();
    Ok(data)
}

pub fn extract_database_changes_from_map(data: BlockScopedData, module_name: String) -> Result<DatabaseChanges, Error> {
    let output = data.output.as_ref().unwrap().map_output.as_ref().unwrap();

    use prost::Message;
    let data: DatabaseChanges = Message::decode(output.value.as_slice()).unwrap();
    Ok(data)

    // match output.data.as_ref().unwrap() {
    //     MapOutput(data) => {
    //         let wrapper: DatabaseChanges = Message::decode(data.value.as_slice())?;
    //         Ok(wrapper)
    //     }
    //     _ => {
    //         Err(format_err!("invalid module output StoreDeltas, expecting MapOutput"))
    //     }
    // }
}

pub fn map_exchange_to_trade(exchange: ProcessExchange, symbol_store: Arc<SymbolStore>) -> Result<Trade, Error> {
    let mut trade = Trade {
        pk: exchange.pk,
        symbol: "-none-".to_string(),
        signature: exchange.signature,
        block: exchange.block as i64,
        timestamp: exchange.timestamp,
        timestamp_ts: NaiveDateTime::from_timestamp_millis(exchange.timestamp * 1000).unwrap(),
        order_taker: exchange.order_taker,
        currency_mint: exchange.currency_mint,
        asset_mint: exchange.asset_mint,
        order_initializer: exchange.order_initializer,
        asset_receiving_wallet: exchange.asset_receiving_wallet,
        asset_change: exchange.asset_change.parse::<f64>().unwrap(),
        currency_change: exchange.currency_change.parse::<f64>().unwrap(),
        market_fee: exchange.market_fee.parse::<f64>().unwrap(),
        price: exchange.price.parse::<f64>().unwrap(),
        total_cost: exchange.total_cost.parse::<f64>().unwrap(),
    };

    trade.symbol = match symbol_store
        .assets
        .clone()
        .into_iter()
        .find(|asset| { asset.mint == trade.asset_mint && asset.pair_mint == trade.currency_mint })
    {
        None => {
            panic!("Error no symbol fetching implemented! - fail here and restart the service while fetching missing symbols!");
        }
        Some(asset) => { asset.symbol }
    };

    return Ok(trade);
}

pub fn map_trade_to_struct(table_change: TableChange, symbol_store: Arc<SymbolStore>) -> Result<Trade, Error> {
    let block_time = table_change.clone().fields.into_iter().find(|t| { t.name == "timestamp" }).ok_or("timestamp").unwrap().new_value.parse::<i64>().unwrap();
    let mut trade = Trade {
        pk: "-none-".to_string(),
        symbol: "-none-".to_string(),
        signature: table_change.clone().fields.into_iter().find(|t| { t.name.contains("signature") }).unwrap().new_value,
        block: table_change.clone().fields.into_iter().find(|t| { t.name.contains("block") }).unwrap().new_value.parse().unwrap_or(0),
        timestamp: block_time,
        timestamp_ts: NaiveDateTime::from_timestamp_millis(block_time * 1000).unwrap(),
        order_taker: table_change.clone().fields.into_iter().find(|t| { t.name == "order_taker" }).ok_or("order_taker").unwrap().new_value,
        currency_mint: table_change.clone().fields.into_iter().find(|t| { t.name == "currency_mint" }).ok_or("currency_mint").unwrap().new_value,
        asset_mint: table_change.clone().fields.into_iter().find(|t| { t.name == "asset_mint" }).ok_or("asset_mint").unwrap().new_value,
        order_initializer: table_change.clone().fields.into_iter().find(|t| { t.name == "order_initializer" }).ok_or("order_initializer").unwrap().new_value,
        asset_receiving_wallet: table_change.clone().fields.into_iter().find(|t| { t.name == "asset_receiving_wallet" }).ok_or("asset_receiving_wallet").unwrap().new_value,
        asset_change: table_change.clone().fields.into_iter().find(|t| { t.name == "asset_change" }).ok_or("asset_change").unwrap().new_value.parse().unwrap_or(0.0),
        currency_change: table_change.clone().fields.into_iter().find(|t| { t.name == "currency_change" }).ok_or("currency_change").unwrap().new_value.parse().unwrap_or(0.0),
        market_fee: table_change.clone().fields.into_iter().find(|t| { t.name == "market_fee" }).ok_or("market_fee").unwrap().new_value.parse().unwrap_or(0.0),
        price: table_change.clone().fields.into_iter().find(|t| { t.name == "price" }).ok_or("price").unwrap().new_value.parse().unwrap_or(0.0),
        total_cost: table_change.clone().fields.into_iter().find(|t| { t.name == "total_cost" }).ok_or("total_cost").unwrap().new_value.parse().unwrap_or(0.0),
    };

    trade.symbol = match symbol_store
        .assets
        .clone()
        .into_iter()
        .find(|asset| { asset.mint == trade.asset_mint && asset.pair_mint == trade.currency_mint })
    {
        None => {
            //TODO: Fetch metadata from onchain gain
            // let asset_symbol = request_metadata_symbol(
            //     "https://api.mainnet-beta.solana.com".to_string(),
            //     trade.asset_mint.clone());
            // let currency_symbol = request_metadata_symbol(
            //     "https://api.mainnet-beta.solana.com".to_string(),
            //     trade.currency_mint.clone());
            panic!("Error no symbol fetching implemented! - fail here and restart the service while fetching missing symbols");

            let asset_symbol = "unknown".to_string();
            let currency_symbol = "unknown".to_string();


            format!("{}/{}", asset_symbol, currency_symbol)
        }
        Some(asset) => { asset.symbol }
    };

    return Ok(trade);
}
