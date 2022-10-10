use mongodb::bson::doc;
use mongodb::bson::Document;
pub fn get_history_aggregation_countback(
    symbol: String,
    to: u64,
    countback: u64,
    resolution_minute: i64,
) -> Vec<Document> {
    println!("{}", resolution_minute);
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
                "price": doc! {
                    "$divide": [
                    {
                        "$sum": "$exchange.currency_amount"
                    }, {
                        "$sum": "$exchange.token_amount"
                    }
                ]
                },
                "volume": doc! {
                    "$sum": "$exchange.token_amount"
                }
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
        doc! {
            "$limit": countback as i64
        },
    ]
    .to_vec()
}
