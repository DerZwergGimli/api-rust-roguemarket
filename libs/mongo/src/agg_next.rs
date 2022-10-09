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
            "$sort":  {
                "timestamp": -1
            }
        },
        doc! {
            "$match":  {
                "timestamp":  {
                    "$lt": next as i64
                }
            }
        },
        doc! {
            "$limit": 1
        },
    ]
    .to_vec()
}
