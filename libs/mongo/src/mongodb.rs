use crate::agg_history::get_history_aggregation;
use crate::agg_history_countback::get_history_aggregation_countback;
use crate::agg_next::get_history_aggregation_next;
use futures::stream::{StreamExt, TryStreamExt};
use log::warn;
use mongodb::bson::{doc, Bson, Document};
use mongodb::options::{ClientOptions, IndexOptions};
use mongodb::{bson, Client, Collection, Database, IndexModel};
use types::databasetrade::DBTrade;
use types::m_ohclvt::M_OHCLVT;

pub struct MongoDBConnection {
    client: Client,
    db: Database,
    pub collection: Collection<DBTrade>,
}

impl MongoDBConnection {
    pub async fn new(mongourl: &str) -> MongoDBConnection {
        let mut client_options = ClientOptions::parse(mongourl)
            .await
            .expect("Error while setting Database options");
        client_options.app_name = Some("DBRustConnection".to_string());

        let options = IndexOptions::builder().unique(true).build();
        let model_sig = IndexModel::builder()
            .keys(doc! {"signature": 1})
            .options(options)
            .build();
        let model_sym = IndexModel::builder().keys(doc! {"symbol": 1}).build();
        let model_ts = IndexModel::builder().keys(doc! {"timestamp": 1}).build();

        let client = Client::with_options(client_options).expect("Error connecting to Database");
        let db = client.database("galacticMarket");
        let collection = db.collection::<DBTrade>("trades");

        collection.create_index(model_sig, None).await;
        collection.create_index(model_sym, None).await;
        collection.create_index(model_ts, None).await;

        MongoDBConnection {
            client,
            db,
            collection,
        }
    }

    pub async fn insert_db_trade(&self, db_trades: &Vec<DBTrade>) -> usize {
        let mut inserted = 0;

        for db_trade in db_trades.iter() {
            let result = self.collection.insert_one(db_trade, None).await;
            match result {
                Ok(_) => inserted = inserted + 1,
                Err(_) => {}
            }
        }
        return inserted;
    }
}

pub async fn find_udf_trades(
    collection: Collection<DBTrade>,
    symbol: String,
    from: u64,
    to: u64,
    resolution_sec: i64,
    countback: Option<u64>,
) -> Option<Vec<M_OHCLVT>> {
    let mut data: Vec<M_OHCLVT> = Vec::new();
    match countback {
        Some(count) => {
            match collection
                .aggregate(
                    get_history_aggregation_countback(symbol, to, count, resolution_sec),
                    None,
                )
                .await
            {
                Ok(mut cursor) => {
                    while let Some(doc) = cursor.try_next().await.unwrap() {
                        data.push(bson::from_document(doc).unwrap());
                    }
                    data.sort_by(|a, b| a.time_last.cmp(&b.time_last));
                    return Some(data);
                }

                Err(_) => None,
            }
        }
        None => {
            match collection
                .aggregate(
                    get_history_aggregation(symbol, from, to, resolution_sec),
                    None,
                )
                .await
            {
                Ok(mut cursor) => {
                    while let Some(doc) = cursor.try_next().await.unwrap() {
                        data.push(bson::from_document(doc).unwrap());
                    }
                    data.sort_by(|a, b| a.time_last.cmp(&b.time_last));
                    return Some(data);
                }

                Err(_) => None,
            }
        }
    }
}

pub async fn find_udf_trade_next(
    collection: Collection<DBTrade>,
    symbol: String,
    next: u64,
) -> Option<DBTrade> {
    match collection
        .aggregate(get_history_aggregation_next(symbol, next), None)
        .await
    {
        Ok(mut cursor) => {
            while let Some(doc) = cursor.try_next().await.unwrap() {
                return Some(bson::from_document::<DBTrade>(doc).unwrap());
            }
            None
        }
        Err(_) => None,
    }
}
