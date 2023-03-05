use mongodb::bson::doc;
use mongodb::bson::Document;

pub fn get_history_aggregation_next(symbol: String, next: u64) -> Vec<Document> {
    [
        doc! {
            "$match":  {
                "symbol": symbol
            }
        },
        doc! {
            "$match":  {
                "timestamp":  {
                    "$lte": next as i64
                }
            }
        },
        doc! {
        "$sort": doc! {
            "timestamp": -1
            }
        },
        doc! {
            "$limit": 1
        },
    ]
        .to_vec()
}
