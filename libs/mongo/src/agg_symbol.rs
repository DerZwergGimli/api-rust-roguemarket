use mongodb::bson::doc;
use mongodb::bson::Document;

pub fn get_by_symbol_aggreation(symbol: String, limit: Option<i64>) -> Vec<Document> {
    let mut max = limit.unwrap_or(1);
    if max > 100 {
        max = 100
    }
    [
        doc! {
            "$match": doc! {
                "symbol": symbol
            }
        },
        doc! {
            "$limit": max
        },
        doc! {
        "$unset": [
            "__v",
            "_id"
            ]
        }
    ]
        .to_vec()
}
