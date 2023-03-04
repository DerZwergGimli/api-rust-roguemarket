use substreams::log;
use {
    crate::option::COption,
    std::convert::TryInto,
    substreams::{errors::Error},
};


/// Instructions supported by the StarAtlas GalacticMarketplace program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum MarketplaceInstruction<'a> {
    ProcessInitializeBuy,
    ProcessExchnage_T1 {
        order_initializer: Vec<u8>,
        market_vars_account: Vec<u8>,
        deposit_mint: Vec<u8>,
        receive_mint: Vec<u8>,
        order_vault_authority: Vec<u8>,
        initializer_deposit_token_account: Vec<u8>,
        initializer_receive_token_account: Vec<u8>,
        order_account: Vec<u8>,
        registered_currency: Vec<u8>,
        open_orders_counter: Vec<u8>,
        system_program: Vec<u8>,
        rent_rent: Vec<u8>,
        token_program_token: Vec<u8>,
        price: u64,
        origination_qty: u64,

    },
    ProcessInitializeSell,

    InitializeOpenOrdersCounter,

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
            43 => {
                log::info!("[Instruction] ProcessInitializeSell");
                Self::ProcessInitializeSell {}
            }
            85 => {
                log::info!("[Instruction] ProcessCancel");
                Self::ProcessCancel {}
            }
            112 => {
                log::info!("[Instruction] ProcessExchange");
                Self::ProcessExchange {}
            }
            129 => {
                log::info!("[Instruction] ProcessInitializeBuy");
                Self::ProcessInitializeBuy
            }
            221 => {
                log::info!("[Instruction] InitializeOpenOrdersCounter");
                Self::InitializeOpenOrdersCounter
            }

            _ => {
                log::info!("tag={:?}", tag);
                log::info!("invdaild marketplace instruction");
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