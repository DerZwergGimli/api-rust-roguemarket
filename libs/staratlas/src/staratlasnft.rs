use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize)]
pub struct StarAtlasNft{
    #[serde(rename = "_id")]
    pub id: String,
    pub deactivated: bool,
    pub name: String,
    pub description: String,
    pub image: String,
    pub media: Media,
    pub attributes: Attributes,
    pub symbol: String,
    pub markets: Vec<Market>,
    #[serde(rename = "totalSupply")]
    pub total_supply: Option<i64>,
    pub mint: String,
    pub network: Option<Network>,
    #[serde(rename = "tradeSettings")]
    pub trade_settings: TradeSettings,
    pub airdrops: Vec<Airdrop>,
    #[serde(rename = "primarySales")]
    pub primary_sales: Vec<PrimarySale>,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub collection: Option<Collection>,
    pub slots: Option<Slots>,
    #[serde(rename = "id")]
    pub star_atlas_nft_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "__v")]
    pub v: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Airdrop {
    #[serde(rename = "_id")]
    pub id: String,
    pub supply: i64,
    #[serde(rename = "id")]
    pub airdrop_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attributes {
    #[serde(rename = "itemType")]
    pub item_type: ItemType,
    pub tier: Option<i64>,
    pub class: String,
    pub category: Option<String>,
    pub score: Option<i64>,
    pub rarity: Rarity,
    pub musician: Option<String>,
    pub spec: Option<String>,
    pub make: Option<String>,
    pub model: Option<String>,
    #[serde(rename = "unitLength")]
    pub unit_length: Option<f64>,
    #[serde(rename = "unitWidth")]
    pub unit_width: Option<f64>,
    #[serde(rename = "unitHeight")]
    pub unit_height: Option<f64>,
    #[serde(rename = "seriesName")]
    pub series_name: Option<String>,
    pub episode: Option<i64>,
    pub edition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub family: Family,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Market {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    #[serde(rename = "id")]
    pub market_id: String,
    #[serde(rename = "quotePair")]
    pub quote_pair: QuotePair,
    #[serde(rename = "serumProgramId")]
    pub serum_program_id: Option<SerumProgramId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    #[serde(rename = "qrInstagram")]
    pub qr_instagram: Option<String>,
    #[serde(rename = "qrFacebook")]
    pub qr_facebook: Option<String>,
    pub sketchfab: Option<String>,
    pub audio: Option<String>,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
    pub gallery: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimarySale {
    #[serde(rename = "listTimestamp")]
    pub list_timestamp: i64,
    #[serde(rename = "id")]
    pub primary_sale_id: Option<String>,
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub supply: Option<i64>,
    pub price: Option<f64>,
    #[serde(rename = "isMinted")]
    pub is_minted: Option<bool>,
    #[serde(rename = "isListed")]
    pub is_listed: Option<bool>,
    #[serde(rename = "mintTimestamp")]
    pub mint_timestamp: Option<i64>,
    #[serde(rename = "orderId")]
    pub order_id: Option<serde_json::Value>,
    #[serde(rename = "expireTimestamp")]
    pub expire_timestamp: Option<i64>,
    #[serde(rename = "targetPair")]
    pub target_pair: Option<QuotePair>,
    #[serde(rename = "quotePrice")]
    pub quote_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slots {
    #[serde(rename = "crewSlots")]
    pub crew_slots: Option<Vec<Slot>>,
    #[serde(rename = "componentSlots")]
    pub component_slots: Option<Vec<Slot>>,
    #[serde(rename = "moduleSlots")]
    pub module_slots: Option<Vec<Slot>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    #[serde(rename = "type")]
    pub slot_type: String,
    pub size: Size,
    pub quantity: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeSettings {
    #[serde(rename = "expireTime")]
    pub expire_time: Option<ETime>,
    #[serde(rename = "saleTime")]
    pub sale_time: Option<ETime>,
    pub vwap: f64,
    pub msrp: Option<Msrp>,
    #[serde(rename = "saleType")]
    pub sale_type: Option<String>,
    pub limited: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msrp {
    pub value: f64,
    #[serde(rename = "currencySymbol")]
    pub currency_symbol: QuotePair,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ETime {
    Integer(i64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "access")]
    Access,
    #[serde(rename = "collectible")]
    Collectible,
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "ship")]
    Ship,
    #[serde(rename = "story")]
    Story,
    #[serde(rename = "structure")]
    Structure,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Rarity {
    #[serde(rename = "anomaly")]
    Anomaly,
    #[serde(rename = "common")]
    Common,
    #[serde(rename = "epic")]
    Epic,
    #[serde(rename = "legendary")]
    Legendary,
    #[serde(rename = "rare")]
    Rare,
    #[serde(rename = "uncommon")]
    Uncommon,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Family {
    #[serde(rename = "Star Atlas")]
    StarAtlas,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QuotePair {
    #[serde(rename = "ATLAS")]
    Atlas,
    #[serde(rename = "SOL")]
    Sol,
    #[serde(rename = "USDC")]
    Usdc,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SerumProgramId {
    #[serde(rename = "srmv4uTCPF81hWDaPyEN2mLZ8XbvzuEM6LsAxR8NpjU")]
    Srmv4UTcpf81HWDaPyEn2MLz8XbvzuEm6LsAxR8NpjU,
    #[serde(rename = "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin")]
    The9XQeWvG816BUx9EPjHmaT23YvVm2ZWbrrpZb9PusVFin,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Network {
    #[serde(rename = "mainnet-beta")]
    MainnetBeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Size {
    #[serde(rename = "capital")]
    Capital,
    #[serde(rename = "commander")]
    Commander,
    #[serde(rename = "crew")]
    Crew,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "Large")]
    SizeLarge,
    #[serde(rename = "XX-Small")]
    SizeXxSmall,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "titan")]
    Titan,
    #[serde(rename = "x-small")]
    XSmall,
    #[serde(rename = "xx-small")]
    XxSmall,
    #[serde(rename = "xxx-small")]
    XxxSmall,
}
