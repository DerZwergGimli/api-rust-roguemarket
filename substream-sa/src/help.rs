use std::collections::HashMap;

use substreams::log;
use substreams_solana::pb::sol::v1::{CompiledInstruction, InnerInstructions, TransactionStatusMeta};

pub fn calc_token_decimals(value: u64, mint: String) -> f64 {
    let mut store = HashMap::new();
    store.insert("ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx", 8);
    store.insert("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", 6);


    (value as f64) / (10.0_f64).powi(*store.get(mint.as_str()).unwrap())
}

pub fn calc_token_balance_change(meta: &TransactionStatusMeta, currency_mint: String, authority: String) -> f64 {
    (meta
        .clone().post_token_balances
        .into_iter()
        .find(|tb| { tb.mint == currency_mint && tb.owner == authority })
        .unwrap_or_default()
        .ui_token_amount
        .unwrap_or_default()
        .ui_amount -
        meta.clone().pre_token_balances
            .into_iter()
            .find(|tb| { tb.mint == currency_mint && tb.owner == authority })
            .unwrap_or_default()
            .ui_token_amount
            .unwrap_or_default()
            .ui_amount).abs()
}

pub fn find_asset_mint_in_inner_instruction_get_index(inner_instructions: InnerInstructions, asset_mint_account: u8) -> Option<usize> {
    //Todo: Check if this works
    log::info!("{:?}", inner_instructions);
    for (idx, instruction) in inner_instructions.instructions.clone().into_iter().enumerate() {
        if instruction.accounts.contains(&asset_mint_account) {
            return Some(idx);
        }
    }

    None
}


pub fn db_change_create(value: &str) -> (&str, &str) {
    return (value, value);
}