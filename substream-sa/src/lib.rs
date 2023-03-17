use serde_json::json;
use substreams::{log, proto};
use substreams::errors::Error;
use substreams::Hex;
use substreams::store::{DeltaProto, Deltas, StoreGet, StoreGetProto, StoreSet, StoreSetProto};
use substreams::store::StoreNew;
use substreams_database_change::pb::database::{DatabaseChanges, TableChange};
use substreams_database_change::pb::database::table_change::Operation;
use substreams_solana::pb::sol;
use substreams_solana::pb::sol::v1::Block;

use crate::help::{calc_token_balance_change, calc_token_decimals, db_change_create};
use crate::pb::trade::ProcessExchange;
use crate::sa_instruction::MarketplaceInstruction;

mod pb;
mod help;
mod sa_instruction;
mod option;


#[substreams::handlers::map]
fn map_sa_trades(blk: sol::v1::Block) -> Result<pb::trade::ProcessExchanges, Error> {
    log::info!("map_sa_trades");
    let mut process_exchanges = vec![];
    process_blocks(blk, &mut process_exchanges).unwrap();

    return Ok(pb::trade::ProcessExchanges { process_exchanges });
}


#[substreams::handlers::store]
fn store_sa_trades(blk: sol::v1::Block, output: StoreSetProto<pb::trade::ProcessExchange>) {
    log::info!("store_sa_trades");

    let mut process_exchanges = vec![];
    process_blocks(blk, &mut process_exchanges).unwrap();


    for exchange in process_exchanges {
        output.set(0, exchange.clone().signature, &exchange);
    }
}

#[substreams::handlers::map]
fn db_out(exchanges: pb::trade::ProcessExchanges) -> Result<DatabaseChanges, Error> {
    log::info!("db_sa_trades");

    substreams::register_panic_hook();
    let mut database_changes: DatabaseChanges = Default::default();

    for exchange in exchanges.process_exchanges {
        database_changes.push_change("trade", exchange.signature.as_str(), 0, Operation::Create)
            .change("block", db_change_create(format!("{:}", exchange.block).as_str()))
            .change("timestamp", db_change_create(format!("{:}", exchange.timestamp).as_str()))
            .change("order_taker", db_change_create(format!("{:}", exchange.order_taker).as_str()))
            .change("currency_mint", db_change_create(format!("{:}", exchange.currency_mint).as_str()))
            .change("asset_mint", db_change_create(format!("{:}", exchange.asset_mint).as_str()))
            .change("order_initializer", db_change_create(format!("{:}", exchange.order_initializer).as_str()))
            .change("asset_change", db_change_create(format!("{:}", exchange.asset_change).as_str()))
            .change("market_fee", db_change_create(format!("{:}", exchange.market_fee).as_str()))
            .change("total_cost", db_change_create(format!("{:}", exchange.total_cost).as_str()));
    }
    return Ok(database_changes);
}


#[substreams::handlers::map]
fn db_sa_trades(blk: sol::v1::Block) -> Result<DatabaseChanges, Error> {
    log::info!("db_sa_trades");

    substreams::register_panic_hook();
    let mut process_exchanges = vec![];
    process_blocks(blk, &mut process_exchanges).unwrap();


    let mut database_changes: DatabaseChanges = Default::default();

    for exchange in process_exchanges {
        database_changes.push_change("trade", exchange.signature.as_str(), 0, Operation::Create)
            .change("block", db_change_create(format!("{:}", exchange.block).as_str()))
            .change("timestamp", db_change_create(format!("{:}", exchange.timestamp).as_str()))
            .change("order_taker", db_change_create(format!("{:}", exchange.order_taker).as_str()))
            .change("currency_mint", db_change_create(format!("{:}", exchange.currency_mint).as_str()))
            .change("asset_mint", db_change_create(format!("{:}", exchange.asset_mint).as_str()))
            .change("order_initializer", db_change_create(format!("{:}", exchange.order_initializer).as_str()))
            .change("asset_change", db_change_create(format!("{:}", exchange.asset_change).as_str()))
            .change("market_fee", db_change_create(format!("{:}", exchange.market_fee).as_str()))
            .change("total_cost", db_change_create(format!("{:}", exchange.total_cost).as_str()))
            .change("price", db_change_create(format!("{:}", exchange.price).as_str()));
    }
    return Ok(database_changes);
}

#[substreams::handlers::map]
fn sa_trades_db_out(store: StoreGetProto<pb::trade::ProcessExchanges>) -> Result<DatabaseChanges, Error> {
    log::info!("sa_trades_db_out");


    let mut database_changes: DatabaseChanges = Default::default();


    // for trx in blk.transactions {
    //     for transaction in trx.transaction {
    //         match store.must_get_last(bs58::encode(transaction.signatures[0].as_slice()).into_string()) {
    //             None => { continue; }
    //             Some(exchange) => {
    //                 database_changes.push_change("trade", exchange.signature.as_str(), 0, Operation::Create)
    //                     .change("block", db_change_create(format!("{:}", exchange.block).as_str()))
    //                     .change("timestamp", db_change_create(format!("{:}", exchange.timestamp).as_str()))
    //                     .change("order_taker", db_change_create(format!("{:}", exchange.order_taker).as_str()))
    //                     .change("currency_mint", db_change_create(format!("{:}", exchange.currency_mint).as_str()))
    //                     .change("asset_mint", db_change_create(format!("{:}", exchange.asset_mint).as_str()))
    //                     .change("order_initializer", db_change_create(format!("{:}", exchange.order_initializer).as_str()))
    //                     .change("asset_change", db_change_create(format!("{:}", exchange.asset_change).as_str()))
    //                     .change("market_fee", db_change_create(format!("{:}", exchange.market_fee).as_str()))
    //                     .change("total_cost", db_change_create(format!("{:}", exchange.total_cost).as_str()));
    //             }
    //         }
    //     }
    // }
    return Ok(database_changes);
}

fn process_blocks(blk: Block, process_exchanges: &mut Vec<ProcessExchange>) -> Result<(), Error> {
    for trx in blk.transactions {
        if let Some(meta) = trx.meta {
            if let Some(_) = meta.err {
                continue;
            }
            if let Some(transaction) = trx.transaction {
                if let Some(msg) = transaction.clone().message {
                    for inst in &msg.instructions {
                        let program_id = &msg.account_keys[inst.program_id_index as usize];
                        if bs58::encode(program_id).into_string() != "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg" {
                            continue;
                        }
                        //Continue Parsing
                        let sig = bs58::encode(transaction.signatures[0].as_slice()).into_string();
                        log::info!("{}", sig.clone());

                        let instruction = MarketplaceInstruction::unpack(&inst.data)?;
                        match instruction {
                            MarketplaceInstruction::ProcessExchange { purchase_quantity, expected_price } => {
                                let order_taker = bs58::encode(&msg.account_keys[inst.accounts[0] as usize]).into_string();
                                let currency_mint = bs58::encode(&msg.account_keys[inst.accounts[3] as usize]).into_string();
                                let order_initializer = bs58::encode(&msg.account_keys[inst.accounts[5] as usize]).into_string();
                                let asset_mint = bs58::encode(&msg.account_keys[inst.accounts[4] as usize]).into_string();
                                let current_change_abs =
                                    calc_token_balance_change(&meta, currency_mint.clone(), bs58::encode(&msg.account_keys[inst.accounts[0] as usize]).into_string());
                                let asset_change_abs = purchase_quantity as f64;
                                let fees_change_abs = calc_token_balance_change(&meta, currency_mint.clone(), "feesQYAaH3wjGUUQYD959mmi5pY8HSz3F5C3SVc1fp3".to_string());

                                process_exchanges.push(pb::trade::ProcessExchange {
                                    pk: format!("{}_{}_{}", sig.clone(), order_taker.clone(), order_initializer.clone()),
                                    signature: sig,
                                    block: blk.slot,
                                    timestamp: blk.block_time.clone().unwrap_or_default().timestamp,
                                    order_taker,
                                    order_initializer,
                                    currency_mint: currency_mint.clone(),
                                    asset_mint,
                                    asset_change: asset_change_abs.to_string(),
                                    market_fee: fees_change_abs.to_string(),
                                    total_cost: current_change_abs.to_string(),
                                    price: calc_token_decimals(expected_price, currency_mint).to_string(),
                                })
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
