use mongodb::bson::doc;
use mongodb::bson::Document;
pub fn get_by_symbol_aggreation(symbol: String, limit: Option<i64>) -> Vec<Document> {
    [
        doc! {
            "$match": doc! {
                "symbol": symbol
            }
        },
        doc! {
            "$limit": limit.unwrap_or(10)
        },
    ]
    .to_vec()
}
