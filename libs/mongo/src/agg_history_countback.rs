use mongodb::bson::doc;
use mongodb::bson::Document;

pub fn get_history_aggregation_countback(
    symbol: String,
    to: u64,
    countback: u64,
    resultion_second: i64,
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
                "price": {"$divide": ["$total_cost", "$asset_change"]},
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
                            "binSize": resultion_second
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
        doc! {
            "$limit": countback as i64
        },
    ]
        .to_vec()
}
