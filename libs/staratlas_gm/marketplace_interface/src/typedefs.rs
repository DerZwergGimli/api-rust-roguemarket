use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct RoyaltyTier {
    pub stake_amount: u64,
    pub discount: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum OrderSide {
    Buy,
    Sell,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum TokenType {
    Asset,
    Currency,
}
