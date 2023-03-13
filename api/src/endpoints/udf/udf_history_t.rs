use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UdfHistory {
    pub s: String,
    pub t: Vec<i64>,
    pub c: Vec<f64>,
    pub o: Vec<f64>,
    pub h: Vec<f64>,
    pub l: Vec<f64>,
    pub v: Vec<f64>,
}

struct Data {
    timestamp: i64,
    volume: i64,
    price: f64,
}