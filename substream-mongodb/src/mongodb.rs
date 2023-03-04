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

#[derive(Debug, Serialize, Deserialize)]
struct SubstreamsCursor {
    cursor_name: String,
    cursor_value: String,
}

pub async fn database_connect() -> Result<Client, Error> {
    let mut client_options = ClientOptions::parse("mongodb://root:root@localhost:27017").await?;

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

pub async fn database_create(db: Database, element: TableChange) -> Result<(), Error> {
    //Make sure unique key is set
    let collection = db.collection::<Document>(element.table.as_str());
    let index_model = IndexModel::builder()
        .keys(doc! {"signature": 1})
        .options(IndexOptions::builder().unique(true).build())
        .build();

    collection.create_index(index_model, None).await?;


    let signature_exists = match collection.find_one(doc! {
            "signature": element.pk.clone(),
      }, None).await? {
        None => { false }
        Some(cursor) => { true }
    };

    if !signature_exists {
        let mut doc = doc! {};
        doc.insert("signature", element.pk);
        for field in element.fields {
            doc.insert(field.name, field.new_value);
        }
        match collection.insert_one(doc, None).await {
            Ok(InsertOneResult { .. }) => {
                info!("inserted!");
            }
            _ => { return Err(anyhow!("Error adding doc!")); }
        }
    }
    Ok(())
}

pub async fn database_cursor_create(db: Database, cursor_name: String, cursor_value: Option<String>) -> Result<(), Error> {
    return match cursor_value.unwrap() {
        cursor_str => {
            let collection = db.collection::<SubstreamsCursor>("_cursor");
            let db_cursor = SubstreamsCursor {
                cursor_name,
                cursor_value: cursor_str,
            };


            match collection.insert_one(db_cursor, None).await? {
                InsertOneResult { .. } => {
                    info!("Cursor written!");
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