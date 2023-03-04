use substreams_solana::pb::sol::v1::TransactionStatusMeta;

pub fn calc_token_balance_change(meta: &TransactionStatusMeta, currency_mint: String, order_initializer: String) -> f64 {
    (meta.clone().post_token_balances
        .into_iter()
        .find(|tb| { tb.mint == currency_mint && tb.owner == order_initializer })
        .unwrap()
        .ui_token_amount
        .unwrap()
        .ui_amount -
        meta.clone().pre_token_balances
            .into_iter()
            .find(|tb| { tb.mint == currency_mint && tb.owner == order_initializer })
            .unwrap()
            .ui_token_amount
            .unwrap()
            .ui_amount).abs()
}