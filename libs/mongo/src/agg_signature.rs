use mongodb::bson::doc;
use mongodb::bson::Document;
pub fn get_signature_aggregation(signature: String) -> Vec<Document> {
    [
        doc! {
            "$match": doc! {
                "signature": signature
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
