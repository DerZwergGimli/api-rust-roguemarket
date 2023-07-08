use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SymbolKV {
    pub key: String,
    pub symbol: String,
}