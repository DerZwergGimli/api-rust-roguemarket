use std::collections::HashMap;

use substreams_solana::pb::sol::v1::TransactionStatusMeta;

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


pub fn db_change_create(value: &str) -> (&str, &str) {
    return (value, value);
}