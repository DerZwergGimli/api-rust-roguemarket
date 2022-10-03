use crate::staratlasnft::StarAtlasNft;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolStore {
    pub assets: Vec<Asset>,
    pub currencies: Vec<Currency>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub symbol: String,
    pub mint: String,
    pub pair_mint: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Currency {
    pub name: String,
    pub mint: String,
}

pub struct BuilderSymbolStore {}

impl BuilderSymbolStore {
    pub fn new() -> BuilderSymbolStore {
        BuilderSymbolStore {}
    }
    pub async fn init(&self) -> SymbolStore {
        let data = reqwest::get("https://galaxy.staratlas.com/nfts")
            .await
            .unwrap()
            .json::<Vec<StarAtlasNft>>()
            .await
            .unwrap();
        print!("{:?}", data);
        self.create_currencies();
        self.map_data(data)
    }

    fn create_currencies(&self) -> Vec<Currency> {
        let mut currencies: Vec<Currency> = Vec::new();
        currencies.push(Currency {
            name: "USDC".to_string(),
            mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
        });
        currencies.push(Currency {
            name: "ATLAS".to_string(),
            mint: "ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx".to_string(),
        });
        return currencies;
    }

    fn map_data(&self, data: Vec<StarAtlasNft>) -> SymbolStore {
        let mut symbol_store: SymbolStore = SymbolStore {
            assets: vec![],
            currencies: vec![],
        };
        symbol_store.currencies = self.create_currencies();

        data.iter().for_each(|asset| {
            symbol_store.currencies.iter().for_each(|currency| {
                symbol_store.assets.push(Asset {
                    symbol: format!("{}{}", asset.symbol.clone(), currency.name.clone()),
                    mint: asset.mint.clone(),
                    pair_mint: currency.mint.clone(),
                })
            })
        });
        return symbol_store;
    }
}
