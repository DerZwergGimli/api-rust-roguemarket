use log::{error, info, warn};
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_client::rpc_config::RpcTransactionConfig;
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::option_serializer::OptionSerializer;

use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
    EncodedTransactionWithStatusMeta, UiInstruction, UiParsedInstruction, UiTransactionEncoding,
};
use staratlas::symbolstore::SymbolStore;
use std::str::FromStr;
use types::databasetrade::{DBTrade, Exchange};
use types::tokentransafer::TokenTransfer;

pub struct Fetcher {
    pub client: RpcClient,
    pub store: SymbolStore,
}

impl Fetcher {
    pub fn new(url: &str, store: SymbolStore) -> Fetcher {
        Fetcher {
            client: RpcClient::new(url),
            store: store,
        }
    }

    pub fn fetch_signatures(
        &self,
        address: &str,
        limit: Option<usize>,
        before: Option<String>,
    ) -> Vec<RpcConfirmedTransactionStatusWithSignature> {
        let mut l_before = None;
        if before != None {
            l_before = Some(Signature::from_str(before.unwrap().as_str()).unwrap());
        };

        let mut result: Option<Vec<RpcConfirmedTransactionStatusWithSignature>> = None;
        while result == None {
            result = match self.client.get_signatures_for_address_with_config(
                &Pubkey::from_str(address).unwrap(),
                GetConfirmedSignaturesForAddress2Config {
                    before: l_before,
                    until: None,
                    limit: limit,
                    commitment: Some(CommitmentConfig::finalized()),
                },
            ) {
                Ok(data) => Some(data),
                Err(err) => {
                    warn!("{:?}", err.kind);
                    None
                }
            }
        }
        return result.unwrap();
    }

    pub fn fetch_transactions(
        &self,
        signatures: &Vec<RpcConfirmedTransactionStatusWithSignature>,
    ) -> Vec<EncodedConfirmedTransactionWithStatusMeta> {
        let mut transactions = Vec::new();

        signatures.into_iter().for_each(|signature| {
            //Make sure to remove failed TXs
            if signature.err == None {
                transactions.push(self.fetch_transaction(signature.clone()));
            }
        });
        return transactions;
    }

    pub fn fetch_transaction(
        &self,
        signature: RpcConfirmedTransactionStatusWithSignature,
    ) -> EncodedConfirmedTransactionWithStatusMeta {
        let mut result: Option<EncodedConfirmedTransactionWithStatusMeta> = None;
        while result == None {
            result = match self.client.get_transaction_with_config(
                &Signature::from_str(signature.signature.as_str()).unwrap(),
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::JsonParsed),
                    commitment: Some(CommitmentConfig::finalized()),
                    max_supported_transaction_version: None,
                },
            ) {
                Ok(data) => Some(data),
                Err(err) => {
                    warn!("{:?}", err.kind);
                    None
                }
            }
        }
        return result.unwrap();
    }

    pub fn filter_transactions_for_exchange(
        &self,
        transactions: Vec<EncodedConfirmedTransactionWithStatusMeta>,
    ) -> Vec<EncodedConfirmedTransactionWithStatusMeta> {
        let filtered_transactions: Vec<EncodedConfirmedTransactionWithStatusMeta> = transactions;

        //Find only with 'ProcessExhange'
        filtered_transactions
            .into_iter()
            .filter(|transaction| {
                match transaction.transaction.clone().meta.unwrap().log_messages {
                    OptionSerializer::Some(logs) => logs
                        .iter()
                        .any(|log| log.contains(&"ProcessExchange".to_string())),
                    OptionSerializer::None => false,
                    OptionSerializer::Skip => false,
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn map_transactions(
        &self,
        transactions: &Vec<EncodedConfirmedTransactionWithStatusMeta>,
    ) -> Vec<DBTrade> {
        let mut dbtrade: Vec<DBTrade> = Vec::new();

        transactions.into_iter().for_each(|transaction| {
            let signature = self
                .find_signature(transaction.transaction.clone())
                .unwrap_or("failed signature".to_string());

            let mut exchanges = Vec::new();
            match self.find_exchanges(transaction.transaction.clone()) {
                None => {
                    info!("{}", signature)
                }
                Some(ex) => exchanges = ex,
            };
            if exchanges.len() == 0 {
                println!("{}", signature);
            }

            dbtrade.push(DBTrade {
                signature: signature,
                timestamp: transaction.block_time.clone().unwrap_or(0),
                slot: transaction.slot.clone(),
                symbol: self.find_symbol(exchanges.clone()),
                exchange: Some(exchanges),
            })
        });

        return dbtrade;
    }

    fn find_signature(&self, transaction: EncodedTransactionWithStatusMeta) -> Option<String> {
        match transaction.transaction {
            EncodedTransaction::LegacyBinary(_) => None,
            EncodedTransaction::Binary(_, _) => None,
            EncodedTransaction::Json(json) => {
                return Some(json.signatures[0].clone());
            }
            EncodedTransaction::Accounts(_) => None,
        }
    }

    fn find_exchanges(
        &self,
        transaction: EncodedTransactionWithStatusMeta,
    ) -> Option<Vec<Exchange>> {
        let mut exchange: Vec<Exchange> = Vec::new();

        match transaction.meta.unwrap().inner_instructions.clone() {
            OptionSerializer::Some(instructions) => {
                for ins in instructions.iter() {
                    if ins.instructions.len() == 3 {
                        for i in (0..ins.clone().instructions.len()).step_by(3) {
                            match self.to_parsed(Vec::from(&ins.instructions[i..(i + 3)])) {
                                None => {
                                    warn!("not added to array")
                                }
                                Some(parsed) => exchange.push(parsed),
                            };
                        }
                    }
                }
            }
            OptionSerializer::None => {}
            OptionSerializer::Skip => {}
        }
        return Some(exchange);
    }

    fn to_parsed(&self, instructions: Vec<UiInstruction>) -> Option<Exchange> {
        if instructions.len() == 3 {
            let mut token_transfers: Vec<TokenTransfer> = Vec::new();

            //Cast Some to Json(Vec<TokenTransfer>)
            instructions.iter().for_each(|ins| match ins {
                UiInstruction::Compiled(_) => {}
                UiInstruction::Parsed(parsed) => match parsed {
                    UiParsedInstruction::PartiallyDecoded(_) => {}
                    UiParsedInstruction::Parsed(p) => {
                        match serde_json::from_value(p.parsed.clone()) {
                            Ok(data) => token_transfers.push(data),
                            Err(err) => {
                                warn!("{}", err)
                            }
                        };
                    }
                },
            });
            if token_transfers.len() == 3 {
                //MAPPING
                //1. Remove 0 Index not needed
                token_transfers.remove(0);

                //2. Find Index of Currency
                let idx_currency = token_transfers
                    .iter()
                    .position(|transfer| {
                        self.store
                            .currencies
                            .clone()
                            .iter()
                            .any(|currency| transfer.info.mint.contains(currency.mint.as_str()))
                    })
                    .unwrap();
                let idx_token = token_transfers
                    .iter()
                    .position(|transfer| {
                        self.store
                            .assets
                            .clone()
                            .iter()
                            .any(|asset| transfer.info.mint.contains(asset.mint.as_str()))
                    })
                    .unwrap();

                return Some(Exchange {
                    side: idx_token == 1,
                    seller: token_transfers[idx_currency].info.authority.clone(),
                    buyer: token_transfers[idx_token].info.authority.clone(),
                    currency_mint: token_transfers[idx_currency].info.mint.clone(),
                    token_mint: token_transfers[idx_token].info.mint.clone(),
                    currency_amount: token_transfers[idx_currency]
                        .info
                        .token_amount
                        .ui_amount
                        .clone(),
                    token_amount: token_transfers[idx_token]
                        .info
                        .token_amount
                        .ui_amount
                        .clone(),
                });
            } else {
                warn!("token transfer not len 3");
                return None;
            }
        } else {
            warn!("Instructions length is not 3")
        }
        return None;
    }

    fn find_symbol(&self, exchanges: Vec<Exchange>) -> String {
        let symbol = self.store.assets.clone().into_iter().find(|asset| {
            asset.mint == exchanges[0].token_mint && asset.pair_mint == exchanges[0].currency_mint
        });
        return symbol.unwrap().symbol;
    }
}
