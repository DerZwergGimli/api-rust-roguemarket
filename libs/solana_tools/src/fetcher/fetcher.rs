use std::str::FromStr;

use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, EncodedTransactionWithStatusMeta, UiTransactionEncoding};
use solana_transaction_status::option_serializer::OptionSerializer;
use types::DBTrade;

pub struct Fetcher {
    pub client: RpcClient,

}

impl Fetcher {
    pub fn new(&mut self) {
        let url = "https://ssc-dao.genesysgo.net/".to_string();
        self.client = RpcClient::new(url);
    }

    pub fn fetch_signatures(&self, address: &str, limit: Option<usize>) -> Vec<RpcConfirmedTransactionStatusWithSignature> {
        let signatures = self
            .client.get_signatures_for_address_with_config(&Pubkey::from_str(address).unwrap(), GetConfirmedSignaturesForAddress2Config {
            before: None,
            until: None,
            limit: limit,
            commitment: None,
        })
            .expect("Unable to get signatures");
        return signatures;
    }

    pub fn fetch_transactions(&self, signatures: &Vec<RpcConfirmedTransactionStatusWithSignature>) -> Vec<EncodedConfirmedTransactionWithStatusMeta> {
        let mut transactions = Vec::new();

        signatures.into_iter().for_each(
            |signature| {
                transactions.push(self
                    .client
                    .get_transaction(&Signature::from_str(signature.signature.as_str()).unwrap(), UiTransactionEncoding::JsonParsed)
                    .expect("Error to get transactions"));
            }
        );
        return transactions;
    }
    pub fn filter_transactions_forExchange(&self, transactions: &Vec<EncodedConfirmedTransactionWithStatusMeta>) -> Vec<EncodedTransactionWithStatusMeta> {
        let mut filtered_transactions: Vec<EncodedTransactionWithStatusMeta> = Vec::new();
        transactions
            .into_iter()
            .for_each(|transaction|
                {
                    filtered_transactions.push(transaction.transaction.clone())
                }
            );


        filtered_transactions
            .into_iter()
            .filter(|transaction| {
                match transaction.clone().meta.unwrap().log_messages {
                    OptionSerializer::Some(logs) => { logs.iter().any(|log| log.contains(&"ProcessExchange".to_string())) }
                    OptionSerializer::None => { false }
                    OptionSerializer::Skip => { false }
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn map_transactions(&self, transactions: &Vec<EncodedTransactionWithStatusMeta>) -> DBTrade {
        let mut dbtrade: Vec<DBTrade> = Vec::new();
        transactions
            .into_iter()
            .for_each(|transaction|
                dbtrade.push(DBTrade {
                    signature: self.find_signature(transaction.clone()).unwrap_or("failed signature".to_string()),
                    timestamp: 0,
                })
            );

        return DBTrade;
    }

    fn find_signature(&self, transaction: EncodedTransactionWithStatusMeta) -> Option<String> {
        match transaction.transaction {
            EncodedTransaction::LegacyBinary(_) => { None }
            EncodedTransaction::Binary(_, _) => { None }
            EncodedTransaction::Json(json) => {
                return Some(json.signatures[0].clone());
            }
            EncodedTransaction::Accounts(_) => { None }
        }
    }
}