use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum MarketplaceError {
    #[error("Invalid Destination Token Account")]
    InvalidDestinationAccount = 6000u32,
    #[error("Invalid instruction.")]
    InvalidInstruction = 6001u32,
    #[error("Invalid SPL Token mint")]
    InvalidMint = 6002u32,
    #[error("Invalid Offer Account Owner")]
    InvalidOfferAccountOwner = 6003u32,
    #[error("Invalid SPL Token account")]
    InvalidTokenAccount = 6004u32,
    #[error("Numerical overflow error")]
    NumericalOverflowError = 6005u32,
    #[error("Invalid Update Authority account")]
    InvalidUpdateAuthorityAccount = 6006u32,
    #[error("Invalid Order Vault Authority account")]
    InvalidOrderVaultAuthorityAccount = 6007u32,
    #[error("Uninitialized Token Account")]
    UninitializedTokenAccount = 6008u32,
    #[error("Insufficient Balance")]
    InsufficientBalance = 6009u32,
    #[error("Invalid Order Duration")]
    InvalidOrderDuration = 6010u32,
    #[error("Origination quantity must be greater than 0")]
    InvalidOriginationQty = 6011u32,
    #[error("Insufficient Order Quantity Remaining")]
    InsufficientOrderQty = 6012u32,
    #[error("Invalid Royalty Value")]
    InvalidRoyalty = 6013u32,
    #[error("Invalid Open Order Counter")]
    InvalidCounter = 6014u32,
    #[error("Mint must be zero decimal")]
    MintDecimalError = 6015u32,
    #[error("Order Account does not match provided account")]
    InvalidOrderAccountError = 6016u32,
    #[error("No royalty tier exists with provided stake amount")]
    InvalidRoyaltyTier = 6017u32,
    #[error("Royalty Tier vector cannot hold any additional tiers")]
    RoyaltyTierLength = 6018u32,
    #[error("Order price did not match expected price")]
    InvalidOrderPrice = 6019u32,
    #[error("Royalty tier already exists")]
    DuplicateRoyaltyTier = 6020u32,
    #[error("Order seller did not match expected seller")]
    InvalidSeller = 6021u32,
}
impl From<MarketplaceError> for ProgramError {
    fn from(e: MarketplaceError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for MarketplaceError {
    fn type_of() -> &'static str {
        "MarketplaceError"
    }
}
impl PrintProgramError for MarketplaceError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
