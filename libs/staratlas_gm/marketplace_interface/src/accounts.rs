use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::*;
pub const MARKET_VARS_ACCOUNT_DISCM: [u8; 8] = [255, 142, 134, 25, 56, 1, 219, 124];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct MarketVars {
    pub update_authority_master: Pubkey,
    pub bump: u8,
}
pub const OPEN_ORDERS_COUNTER_ACCOUNT_DISCM: [u8; 8] = [
    245,
    112,
    49,
    129,
    46,
    33,
    183,
    73,
];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct OpenOrdersCounter {
    pub open_order_count: u64,
    pub bump: u8,
}
pub const ORDER_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [79, 67, 112, 155, 214, 14, 32, 55];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct OrderAccount {
    pub order_initializer_pubkey: Pubkey,
    pub currency_mint: Pubkey,
    pub asset_mint: Pubkey,
    pub initializer_currency_token_account: Pubkey,
    pub initializer_asset_token_account: Pubkey,
    pub order_side: OrderSide,
    pub price: u64,
    pub order_origination_qty: u64,
    pub order_remaining_qty: u64,
    pub created_at_timestamp: i64,
}
pub const REGISTERED_CURRENCY_ACCOUNT_DISCM: [u8; 8] = [
    60,
    114,
    244,
    134,
    16,
    166,
    51,
    149,
];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct RegisteredCurrency {
    pub token_mint: Pubkey,
    pub sa_currency_vault: Pubkey,
    pub royalty: u64,
    pub bump: u8,
    pub royalty_tiers: Vec<RoyaltyTier>,
}
