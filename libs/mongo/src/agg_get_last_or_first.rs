use mongodb::bson::doc;
use mongodb::bson::Document;

pub fn get_last_or_first_aggregation(direction: i32) -> Vec<Document> {
    [
        doc! {
        "$sort": doc! {
            "timestamp": direction
        }
    },
        doc! {
        "$limit": 1
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
