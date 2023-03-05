mod pb;
mod help;
mod sa_instruction;
mod option;

use serde_json::json;
use substreams::errors::Error;
use substreams::log;
use substreams_database_change::pb::database::{DatabaseChanges, TableChange};
use substreams_database_change::pb::database::table_change::Operation;
use substreams_ethereum::pb::eth;
use substreams_solana::pb::sol;
use pb::sinkfiles::Lines;
use substreams::Hex;
use crate::help::calc_token_balance_change;
use crate::sa_instruction::MarketplaceInstruction;

#[substreams::handlers::map]
pub fn db_out_eth(blk: eth::v2::Block) -> Result<DatabaseChanges, Error> {
    log::info!("db_out_eth");

    substreams::register_panic_hook();
    let mut database_changes: DatabaseChanges = Default::default();
    database_changes.push_change("test", "pk_test", 0, Operation::Create)
        .change("key1", ("previous1", "value1"))
        .change("key3", ("previous1", "value1"))
        .change("key4", ("previous1", "value1"))
        .change("key2", ("previous2", "value2"));

    return Ok(database_changes);
}

#[substreams::handlers::map]
fn jsonl_out_eth(block: eth::v2::Block) -> Result<Lines, substreams::errors::Error> {
    let header = block.header.as_ref().unwrap();

    Ok(pb::sinkfiles::Lines {
        lines: vec![json!({
            "number": block.number,
            "hash": Hex(&block.hash).to_string(),
            "parent_hash": Hex(&header.parent_hash).to_string(),
            "timestamp": header.timestamp.as_ref().unwrap().to_string()
        })
            .to_string()],
    })
}

#[substreams::handlers::map]
pub fn db_out_sol(blk: sol::v1::Block) -> Result<DatabaseChanges, Error> {
    log::info!("db_out_sol");

    substreams::register_panic_hook();
    let mut database_changes: DatabaseChanges = Default::default();
    database_changes.push_change("test", "pk_test", 0, Operation::Create)
        .change("key1", ("previous1", "value1"))
        .change("key3", ("previous1", "value1"))
        .change("key4", ("previous1", "value1"))
        .change("key2", ("previous2", "value2"));

    return Ok(database_changes);
}

#[substreams::handlers::map]
fn map_sa_trades(blk: sol::v1::Block) -> Result<pb::trade::ProcessExchanges, Error> {
    let mut process_exchanges = vec![];
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
                        let instruction = MarketplaceInstruction::unpack(&inst.data)?;
                        match instruction {
                            MarketplaceInstruction::ProcessExchange {} => {
                                let order_taker = bs58::encode(&msg.account_keys[inst.accounts[0] as usize]).into_string();
                                let currency_mint = bs58::encode(&msg.account_keys[inst.accounts[3] as usize]).into_string();
                                let order_initializer = bs58::encode(&msg.account_keys[inst.accounts[5] as usize]).into_string();
                                let asset_mint = bs58::encode(&msg.account_keys[inst.accounts[4] as usize]).into_string();
                                let current_change_abs =
                                    calc_token_balance_change(&meta, currency_mint.clone(), order_taker.clone());
                                let asset_change_abs =
                                    calc_token_balance_change(&meta, asset_mint.clone(), order_taker.clone());
                                let fees_change_abs = calc_token_balance_change(&meta, currency_mint.clone(), "feesQYAaH3wjGUUQYD959mmi5pY8HSz3F5C3SVc1fp3".to_string());

                                process_exchanges.push(pb::trade::ProcessExchange {
                                    signature: sig,
                                    block: blk.slot,
                                    timestamp: blk.block_time.clone().unwrap_or_default().timestamp,
                                    order_taker,
                                    order_initializer,
                                    currency_mint,
                                    asset_mint,
                                    asset_change: asset_change_abs.to_string(),
                                    market_fee: fees_change_abs.to_string(),
                                    total_cost: current_change_abs.to_string(),
                                })
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    return Ok(pb::trade::ProcessExchanges { process_exchanges });
}

#[substreams::handlers::map]
fn db_sa_trades(blk: sol::v1::Block) -> Result<DatabaseChanges, Error> {
    log::info!("db_sa_trades");

    substreams::register_panic_hook();
    let mut process_exchanges = vec![];
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
                        let instruction = MarketplaceInstruction::unpack(&inst.data)?;
                        match instruction {
                            MarketplaceInstruction::ProcessExchange {} => {
                                let order_taker = bs58::encode(&msg.account_keys[inst.accounts[0] as usize]).into_string();
                                let currency_mint = bs58::encode(&msg.account_keys[inst.accounts[3] as usize]).into_string();
                                let order_initializer = bs58::encode(&msg.account_keys[inst.accounts[5] as usize]).into_string();
                                let asset_mint = bs58::encode(&msg.account_keys[inst.accounts[4] as usize]).into_string();
                                let current_change_abs =
                                    calc_token_balance_change(&meta, currency_mint.clone(), order_taker.clone());
                                let asset_change_abs =
                                    calc_token_balance_change(&meta, asset_mint.clone(), order_taker.clone());
                                let fees_change_abs = calc_token_balance_change(&meta, currency_mint.clone(), "feesQYAaH3wjGUUQYD959mmi5pY8HSz3F5C3SVc1fp3".to_string());

                                process_exchanges.push(pb::trade::ProcessExchange {
                                    signature: sig,
                                    block: blk.slot,
                                    timestamp: blk.block_time.clone().unwrap_or_default().timestamp,
                                    order_taker,
                                    order_initializer,
                                    currency_mint,
                                    asset_mint,
                                    asset_change: asset_change_abs.to_string(),
                                    market_fee: fees_change_abs.to_string(),
                                    total_cost: current_change_abs.to_string(),
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }


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
            .change("total_cost", db_change_create(format!("{:}", exchange.total_cost).as_str()));
    }
    return Ok(database_changes);
}

fn db_change_create(value: &str) -> (&str, &str) {
    return (value, value);
}