pub mod default;
pub mod udf;
pub mod trades;
pub mod stats;


struct Data {
    timestamp: i64,
    volume: i64,
    price: f64,
}