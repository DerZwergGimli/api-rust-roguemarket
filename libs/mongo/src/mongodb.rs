use mongodb::bson::doc;
use mongodb::options::{ClientOptions, IndexOptions};
use mongodb::results::InsertManyResult;
use mongodb::{Client, Collection, Database, IndexModel};
use types::databasetrade::DBTrade;

pub struct MongoDBConnection {
    client: Client,
    db: Database,
    collection: Collection<DBTrade>,
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

        collection
            .create_index(model_sig, None)
            .await
            .expect("Error setting index-model");

        collection
            .create_index(model_sym, None)
            .await
            .expect("Error setting index-model");

        collection
            .create_index(model_ts, None)
            .await
            .expect("Error setting index-model");

        MongoDBConnection {
            client,
            db,
            collection,
        }
    }

    pub async fn insert_dbTrade(&self, dbTrades: &Vec<DBTrade>) -> usize {
        let mut inserted = 0;

        for db_trade in dbTrades.iter() {
            let result = self.collection.insert_one(db_trade, None).await;
            match result {
                Ok(_) => inserted = inserted + 1,
                Err(err) => {}
            }
        }
        return inserted;
    }
}
