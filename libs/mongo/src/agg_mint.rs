use mongodb::bson::doc;
use mongodb::bson::Document;

pub fn get_mint_aggregation(
    mint: String,
    limit: Option<i64>,
) -> Vec<Document> {
    let mut max = limit.unwrap_or(1);
    if max > 100 {
        max = 100
    }
    [
        doc! {
        "$match": doc! {
            "$or": [
                doc! {
                    "asset_mint": doc! {
                        "$regex": mint.clone()
                    }
                },
                doc! {
                    "currency_mint": doc! {
                        "$regex": mint
                    }
                }
            ]
        }
    },
        doc! {
        "$sort": doc! {
            "timestamp": -1
        }
    },
        doc! {
        "$unset": [
            "__v",
            "_id"
        ]
    },
        doc! {
        "$limit": max
    }
    ]
        .to_vec()
}
