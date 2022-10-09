use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct M_OHCLVT {
    #[serde(rename = "_id")]
    pub id: Id,
    pub time_last: i64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id {
    pub time: Time,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Time {
    #[serde(rename = "$date")]
    pub date: String,
}
