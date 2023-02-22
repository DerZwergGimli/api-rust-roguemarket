use mongodb::bson::doc;
use mongodb::bson::Document;

pub fn get_signature_aggregation(signature: String) -> Vec<Document> {
    [
        doc! {
        "$match": doc! {
            "signature": doc! {
                "$regex": signature
            }
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
    }
    ]
        .to_vec()
}
