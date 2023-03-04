use anyhow::{anyhow, Error};
use log::{error, info, warn};
use mongodb::bson::{doc, Document};
use mongodb::{Client, Database, IndexModel};
use mongodb::error::ErrorKind;
use mongodb::options::{ClientOptions, IndexOptions};
use mongodb::results::{InsertOneResult, UpdateResult};
use tokio::macros;
use crate::pb::database::{DatabaseChanges, TableChange};
use serde::{Deserialize, Serialize};
use progress_bar::*;
use staratlas::symbolstore::SymbolStore;
use crate::trade_t::SATrade;


#[derive(Debug, Serialize, Deserialize)]
struct SubstreamsCursor {
    cursor_name: String,
    cursor_value: String,
}

pub async fn database_connect(url: String) -> Result<Client, Error> {
    let mut client_options = ClientOptions::parse(url).await?;

    client_options.app_name = Some("rust-substream-writer".to_string());
    match Client::with_options(client_options) {
        Ok(db) => {
            info!("DB connected!");
            Ok(db)
        }
        Err(e) => {
            Err(anyhow!("Unable to connect tp DB: {:?}", e))
        }
    }
}

pub async fn database_create(db: Database, element: SATrade, table_name: String) -> Result<(), Error> {

    //Make sure unique key is set
    let collection = db.collection::<SATrade>(table_name.as_str());
    let index_model = IndexModel::builder()
        .keys(doc! {"signature": 1})
        .options(IndexOptions::builder().unique(true).build())
        .build();

    collection.create_index(index_model, None).await?;


    let signature_exists = match collection.find_one(doc! {
            "signature": element.clone().signature,
      }, None).await? {
        None => { false }
        Some(cursor) => { true }
    };

    if !signature_exists {
        match collection.insert_one(element, None).await {
            Ok(InsertOneResult { .. }) => {
                info!("inserted!");
            }
            _ => { return Err(anyhow!("Error adding doc!")); }
        }
    }
    Ok(())
}

pub async fn database_cursor_create(db: Database, cursor_name: String, cursor_value: Option<String>) -> Result<(), Error> {
    let cursor_db = match cursor_value {
        Some(cursor_str) => {
            SubstreamsCursor {
                cursor_name,
                cursor_value: cursor_str,
            }
        }
        _ => {
            SubstreamsCursor {
                cursor_name,
                cursor_value: "".to_string(),
            }
        }
    };

    let collection = db.collection::<SubstreamsCursor>("_cursor");
    match collection.insert_one(cursor_db, None).await? {
        InsertOneResult { .. } => {
            Ok(())
        }
        _ => {
            Err(anyhow!("Error inserting doc"))
        }
    }
}

pub async fn database_cursor_update(db: Database, cursor_name: String, cursor_value: Option<String>) -> Result<(), Error> {
    return match cursor_value.unwrap() {
        cursor_str => {
            let collection = db.collection::<SubstreamsCursor>("_cursor");
            match collection.update_one(doc! {"cursor_name": cursor_name}, doc! {"$set": {"cursor_value": cursor_str}}, None).await? {
                UpdateResult { .. } => {
                    Ok(())
                }
                _ => {
                    Err(anyhow!("Error inserting doc"))
                }
            }
        }
    };
}

pub async fn database_cursor_get(db: Database, cursor_name: String) -> Option<String> {
    let collection = db.collection::<SubstreamsCursor>("_cursor");
    match collection.find_one(doc! {
            "cursor_name": cursor_name
      }, None).await
    {
        Ok(c) => {
            match c {
                None => { None }
                Some(d) => { Some(d.cursor_value.clone()) }
            }
        }
        Err(_) => { None }
    }
}