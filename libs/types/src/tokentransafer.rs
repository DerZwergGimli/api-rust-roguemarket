// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenTransfer {
    pub info: Info,
    #[serde(rename = "type")]
    pub token_transfer_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub authority: String,
    pub destination: String,
    pub mint: String,
    pub source: String,
    #[serde(rename = "tokenAmount")]
    pub token_amount: TokenAmount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAmount {
    pub amount: String,
    pub decimals: i64,
    #[serde(rename = "uiAmount")]
    pub ui_amount: f64,
    #[serde(rename = "uiAmountString")]
    pub ui_amount_string: String,
}
