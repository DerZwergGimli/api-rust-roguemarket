use mongodb::bson::doc;
use mongodb::bson::Document;

pub fn get_history_aggregation(
    symbol: String,
    from: u64,
    to: u64,
    resolution_second: i64,
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
                    "$lte": to as i64,
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
                "price": "$total_cost",
                "volume": "$asset_change"
            }
        },
        doc! {
            "$group": doc! {
                "_id": doc! {
                    "time": doc! {
                        "$dateTrunc": doc! {
                            "date": "$time",
                            "unit": "second",
                            "binSize": resolution_second
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
