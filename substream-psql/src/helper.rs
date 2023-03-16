use std::sync::Arc;

use anyhow::{Error, format_err};
use indicatif::ProgressBar;
use json::object;
use log::info;
use prost::Message;
use reqwest::header;
use staratlas::symbolstore::SymbolStore;

use database_psql::model::Trade;

use crate::pb::database::{DatabaseChanges, TableChange};
use crate::pb::substreams::BlockScopedData;
use crate::pb::substreams::module_output::Data::MapOutput;

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

pub fn extract_database_changes_from_map(data: BlockScopedData, module_name: String) -> Result<DatabaseChanges, Error> {
    let output = data
        .outputs
        .first()
        .ok_or(format_err!("expecting one module output"))?;
    if &output.name != module_name.as_str() {
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

pub fn map_trade_to_struct(table_change: TableChange, symbol_store: Arc<SymbolStore>) -> Result<Trade, Error> {
    let mut trade = Trade {
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
        price: table_change.clone().fields.into_iter().find(|t| { t.name == "price" }).ok_or("price").unwrap().new_value.parse().unwrap_or(0.0),
    };

    trade.symbol = symbol_store
        .assets
        .clone()
        .into_iter()
        .find(|asset| { asset.mint == trade.asset_mint && asset.pair_mint == trade.currency_mint })
        .unwrap()
        .symbol;


    return Ok(trade);
}
