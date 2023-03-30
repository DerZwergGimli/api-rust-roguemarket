// @generated automatically by Diesel CLI.

diesel::table! {
    cursors (id) {
        id -> Text,
        value -> Nullable<Text>,
        block -> Nullable<Int8>,
        start_block -> Nullable<Int8>,
        end_block -> Nullable<Int8>,
    }
}

diesel::table! {
    trades (pk) {
        pk -> Text,
        signature -> Text,
        symbol -> Text,
        block -> Int8,
        timestamp -> Int8,
        timestamp_ts -> Timestamp,
        order_taker -> Text,
        order_initializer -> Text,
        currency_mint -> Text,
        asset_mint -> Text,
        asset_receiving_wallet -> Text,
        asset_change -> Float8,
        market_fee -> Float8,
        total_cost -> Float8,
        price -> Float8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cursors,
    trades,
);
