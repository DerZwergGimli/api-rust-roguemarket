use borsh::BorshDeserialize;
use substreams::errors::Error;
use substreams::log;

use crate::sa_instruction::MarketplaceInstruction::ProcessExchange;

//use borsh::BorshDeserialize;

/// Instructions supported by the StarAtlas GalacticMarketplace program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum MarketplaceInstruction<'a> {
    CreateAccountWithSeed,
    UnknownTransaction,
    //zsucnLZxPfQz2G9aN9BFu75kaXhfYi55UduB9UWJobd81KwaRxudArYgtFD6Yo6tkjnyibRi1VPL5RkwAukD4FY
    UpdateCurrencyVault,
    ProcessInitializeBuy,
    ProcessInitializeSell,
    InitializeOpenOrdersCounter,
    InitializeMarketplace,
    RegisterCurrency,
    UpdateAtlasRate,
    DeregisterCurrency,
    UpdateCurrencyRoyalty,
    ProcessCancel,
    ProcessExchange {
        purchase_quantity: u64,
        expected_price: Option<u64>,
    },
    UiAmountToAmount {
        /// The ui_amount of tokens to reformat.
        ui_amount: &'a str,
    },
}

#[derive(BorshDeserialize, Debug)]
struct Pubkey([u8; 32]);

#[derive(BorshDeserialize, Debug)]
struct ProcessExchangeArgNoPubkeyAndPrice {
    purchase_quantity: u64,
}

#[derive(BorshDeserialize, Debug)]
struct ProcessExchangeArgNoPubkey {
    purchase_quantity: u64,
    expected_price: u64,
}

#[derive(BorshDeserialize, Debug)]
struct ProcessExchangeArgsWithPubkey {
    pub purchase_quantity: u64,
    pub expected_price: u64,
    pub seller: Pubkey,
}


impl<'a> MarketplaceInstruction<'a> {
    pub fn unpack(input: &'a [u8]) -> Result<Self, Error> {
        //Example
        //[112, 194, 63, 99, 52, 147, 85, 48, 1, 0, 0, 0, 0, 0, 0, 0, 128, 139, 121, 2, 0, 0, 0, 0]
        //[112, 194, 63, 99, 52, 147, 85, 48] //PROCESS_EXCHANGE_IX_DISCM
        //                                   [ ARGS                                               ]

        let (&tag, rest) = input.split_first().ok_or(Error::Unexpected(format!("Invalid Instruction")))?;
        let (_dump, exchange_args) = rest.split_at(7);

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
            64 => {
                log::info!("[Instruction] CreateAccountWithSeed");
                Self::CreateAccountWithSeed {}
            }
            112 => {
                log::info!("[Instruction] ProcessExchange");
                log::info!("{:?}", input);
                log::info!("{}", hex::encode(input));
                log::info!("{}", base64::encode(input.clone()));
                log::info!("--------");

                log::info!("{:?}", exchange_args);
                log::info!("{}", hex::encode(exchange_args));
                log::info!("{}", base64::encode(exchange_args.clone()));
                log::info!("--------");

                match exchange_args.len() {
                    8 => {
                        let data = ProcessExchangeArgNoPubkeyAndPrice::try_from_slice(exchange_args).unwrap();
                        log::info!("ProcessExchangeArgNoPubkeyAndPrice={:?}", data);
                        ProcessExchange {
                            purchase_quantity: data.purchase_quantity,
                            expected_price: None,
                        }
                    }
                    16 => {
                        let data = ProcessExchangeArgNoPubkey::try_from_slice(exchange_args).unwrap();
                        log::info!("ProcessExchangeArgNoPubkey={:?}", data);
                        ProcessExchange {
                            purchase_quantity: data.purchase_quantity,
                            expected_price: Some(data.expected_price),
                        }
                    }
                    48 => {
                        let data = ProcessExchangeArgsWithPubkey::try_from_slice(exchange_args).unwrap();
                        log::info!("ProcessExchangeArgsWithPubkey={:?}", data);
                        ProcessExchange {
                            purchase_quantity: data.purchase_quantity,
                            expected_price: Some(data.expected_price),
                        }
                    }
                    _ => {
                        return Err(Error::Unexpected(format!("Invalid args len={:?}", exchange_args.len())));
                    }
                }
            }
            189 => {
                log::info!("[Instruction] DeregisterCurrency");
                Self::DeregisterCurrency {}
            }
            129 => {
                log::info!("[Instruction] ProcessInitializeBuy");
                Self::ProcessInitializeBuy {}
            }
            179 => {
                log::info!("[Instruction] UpdateCurrencyRoyalty");
                Self::UpdateCurrencyRoyalty {}
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
            248 => {
                log::info!("[Instruction] UpdateAtlasRate");
                Self::UpdateAtlasRate {}
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


fn hex_string_to_u32_le_unsafe(s: String) -> Vec<u32> {
    let bytes = hex::decode(s).unwrap();
    let mut ints: Vec<u32> = Vec::new();
    for chunk in bytes.chunks(4) {
        ints.push(u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
    }
    return ints;
}