use {
    crate::option::COption,
    std::convert::TryInto,
    substreams::errors::Error,
};
use substreams::log;

/// Instructions supported by the StarAtlas GalacticMarketplace program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum MarketplaceInstruction<'a> {
    UnknownTransaction,
    //zsucnLZxPfQz2G9aN9BFu75kaXhfYi55UduB9UWJobd81KwaRxudArYgtFD6Yo6tkjnyibRi1VPL5RkwAukD4FY
    UpdateCurrencyVault,
    ProcessInitializeBuy,
    ProcessInitializeSell,
    InitializeOpenOrdersCounter,
    InitializeMarketplace,
    RegisterCurrency,
    DeregisterCurrency,
    ProcessCancel,
    ProcessExchange,
    UiAmountToAmount {
        /// The ui_amount of tokens to reformat.
        ui_amount: &'a str,
    },
}

impl<'a> MarketplaceInstruction<'a> {
    pub fn unpack(input: &'a [u8]) -> Result<Self, Error> {
        let (&tag, rest) = input.split_first().ok_or(Error::Unexpected(format!("Invalid Instruction")))?;


        Ok(match tag {
            18 => {
                log::info!("[Instruction] UpdateCurrencyVault");
                Self::UpdateCurrencyVault {}
            }
            43 => {
                log::info!("[Instruction] ProcessInitializeSell");
                Self::ProcessInitializeSell {}
            }
            47 => {
                log::info!("[Instruction] InitializeMarketplace");
                Self::InitializeMarketplace {}
            }
            74 => {
                log::info!("[Instruction] UnknownTransaction");
                Self::UnknownTransaction {}
            }
            85 => {
                log::info!("[Instruction] ProcessCancel");
                Self::ProcessCancel {}
            }
            112 => {
                log::info!("[Instruction] ProcessExchange");
                Self::ProcessExchange {}
            }
            189 => {
                log::info!("[Instruction] DeregisterCurrency");
                Self::DeregisterCurrency {}
            }
            129 => {
                log::info!("[Instruction] ProcessInitializeBuy");
                Self::ProcessInitializeBuy {}
            }
            221 => {
                log::info!("[Instruction] InitializeOpenOrdersCounter");
                Self::InitializeOpenOrdersCounter {}
            }
            233 => {
                log::info!("[Instruction] UnknownTransaction");
                Self::UnknownTransaction {}
            }
            247 => {
                log::info!("[Instruction] RegisterCurrency");
                Self::RegisterCurrency {}
            }
            _ => {
                log::info!("tag={:?}", tag);
                log::info!("invalid marketplace instruction");
                return Err(Error::Unexpected(format!("Invalid marketplace instruction")));
            }
        })
    }
    fn unpack_pubkey(input: &[u8]) -> Result<(Vec<u8>, &[u8]), Error> {
        if input.len() >= 32 {
            let (key, rest) = input.split_at(32);
            let pk = key.to_vec();
            Ok((pk, rest))
        } else {
            Err(Error::Unexpected(format!("Invalid Instruction")))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InnerInstruction {
    authority: String,
    destination: String,
    mint: String,
    source: String,
    amount: String,
}