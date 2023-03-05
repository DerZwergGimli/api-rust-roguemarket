use mongodb::bson::doc;
use mongodb::bson::Document;

pub fn get_address_aggregation(
    address: String,
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
                    "order_taker": doc! {
                        "$regex": address.clone()
                    }
                },
                doc! {
                    "order_initializer": doc! {
                        "$regex": address
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
