use mongodb::bson::doc;
use mongodb::bson::Document;
pub fn get_history_aggregation(
    symbol: String,
    from: u64,
    to: u64,
    resolution_sec: i64,
) -> Vec<Document> {
    [
        doc! {
            "$match": {
                "trade.symbol": symbol,
            },
        },
        doc! {
            "$match": {
                "timestamp": {"$lt": to, "$gt": from},
            },
        },
        doc! {
            "$addFields": {
                "time": {
                    "$toDate": {
                        "$multiply": ["$timestamp", 1000],
                    },
                },
                "symbol": "$trade.symbol",
                "price": "$trade.cost_price",
                "volume": "$trade.size",
            },
        },
        doc! {
            "$group": {
                "_id": {
                    "time": {
                        "$dateTrunc": {
                            "date": "$time",
                            "unit": "minute",
                            "binSize": resolution_sec,
                        },
                    },
                },
                "time_last": {
                    "$last": "$timestamp",
                },
                "high": {
                    "$max": "$price",
                },
                "low": {
                    "$min": "$price",
                },
                "open": {
                    "$first": "$price",
                },
                "close": {
                    "$last": "$price",
                },
                "volume": {
                    "$sum": "$volume",
                },
            },
        },
        doc! {
            "$sort": {
                "time_last": 1,
            },
        },
    ]
    .to_vec()
}
