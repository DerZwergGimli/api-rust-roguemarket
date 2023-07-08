use log::info;
use serde::{Deserialize, Serialize};

use staratlas_nft::staratlasnft::StarAtlasNft;
use staratlas_symbols::symbol_store::{Asset, Currency, Exchange, SymbolStore};
use staratlas_symbols::symbol_store_kv::SymbolKV;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
        self.create_currencies();

        info!("SA Store initalized!");
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
            exchange: Exchange {
                symbol: "GM".to_string(),
                name: "GalacticMarket".to_string(),
                description: "StarAtlas GalacticMarket".to_string(),
                asset_type: vec!["nft".to_string()],
                sesstion: "24x7".to_string(),
                timezone: "Etc/UTC".to_string(),
                minmovement: 0.0,
                minmov: 1.0,
                minmovement2: 0.0,
                minmov2: 0.0,
                supported_resolutions: vec![
                    "1".to_string(),
                    "3".to_string(),
                    "5".to_string(),
                    "15".to_string(),
                    "30".to_string(),
                    "60".to_string(),
                    "120".to_string(),
                    "240".to_string(),
                    "360".to_string(),
                    "480".to_string(),
                    "720".to_string(),
                    "1D".to_string(),
                    "3D".to_string(),
                    "1W".to_string(),
                    "1M".to_string(),
                ],
                has_intraday: true,
                has_daily: true,
                has_weekly_and_monthly: false,
                data_status: "streaming".to_string(),
                supports_search: true,
                supports_group_request: false,
                supports_marks: false,
                supports_timescale_marks: false,
                supports_time: true,
            },
        };
        symbol_store.currencies = self.create_currencies();

        let mut item_types = Vec::new();
        data.iter().for_each(|asset| {
            symbol_store.currencies.iter().for_each(|currency| {
                if !item_types
                    .clone()
                    .into_iter()
                    .any(|i| i == format!("{:?}", asset.attributes.item_type))
                {
                    item_types.push(format!("{:?}", asset.attributes.item_type))
                }

                symbol_store.assets.push(Asset {
                    asset_name: asset.symbol.clone(),
                    pair_name: currency.name.clone(),
                    description: format!("{} [{}]", asset.name.clone(), currency.name.clone()),
                    asset_type: format!("{:?}", asset.attributes.item_type),
                    symbol: format!("{}{}", asset.symbol.clone(), currency.name.clone()),
                    mint: asset.mint.clone(),
                    pair_mint: currency.mint.clone(),
                    pricescale: 1000000,
                })
            })
        });
        symbol_store.exchange.asset_type = item_types;
        return symbol_store;
    }

    pub async fn simple_json_out(&self) -> Vec<SymbolKV> {
        let store = self.init().await;
        let mut simple_data: Vec<SymbolKV> = vec![];
        store.assets.into_iter().for_each(|asset| {
            simple_data.push(SymbolKV {
                key: format!("{}{}", asset.mint, asset.pair_mint),
                symbol: asset.symbol,
            })
        });
        simple_data
    }
}
