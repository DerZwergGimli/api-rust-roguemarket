use mongodb::bson::doc;
use mongodb::bson::Document;
pub fn get_history_aggregation(
    symbol: String,
    from: u64,
    to: u64,
    resolution_minute: i64,
) -> Vec<Document> {
    [
        doc! {
            "$match": doc! {
                "symbol": symbol
            }
        },
        doc! {
            "$match": doc! {
                "timestamp": doc! {
                    "$lt": to as i64,
                },
                "timestamp": doc! {
                    "$gt": from as i64
                }
            }
        },
        doc! {
            "$addFields": doc! {
                "time": doc! {
                    "$toDate": doc! {
                        "$multiply": [
                            "$timestamp",
                            1000
                        ]
                    }
                },
                "price": "$price",
                "volume": "$size"
            }
        },
        doc! {
            "$group": doc! {
                "_id": doc! {
                    "time": doc! {
                        "$dateTrunc": doc! {
                            "date": "$time",
                            "unit": "minute",
                            "binSize": resolution_minute
                        }
                    }
                },
                "time_last": doc! {
                    "$last": "$timestamp"
                },
                "high": doc! {
                    "$max": "$price"
                },
                "low": doc! {
                    "$min": "$price"
                },
                "open": doc! {
                    "$first": "$price"
                },
                "close": doc! {
                    "$last": "$price"
                },
                "volume": doc! {
                    "$sum": "$volume"
                }
            }
        },
    ]
    .to_vec()
}
