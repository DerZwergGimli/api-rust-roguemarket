mod pb;
mod substreams;
mod substreams_stream;
mod mongodb;


use std::{env, fs};
use std::os::unix::raw::ino_t;
use std::sync::Arc;
use anyhow::{Context, Error, format_err};
use log::{error, info, warn};
use prost::{DecodeError};


use tokio_stream::StreamExt;
use crate::pb::substreams::module_output::Data::MapOutput;
use crate::pb::substreams::{BlockScopedData, Request, StoreDeltas};
use crate::pb::substreams::module_output::Data;
use prost::Message;
use crate::mongodb::{database_connect, database_create, database_cursor_create, database_cursor_get};

use crate::pb::substreams::stream_client::StreamClient;
use crate::pb::substreams::Package;
use crate::substreams::SubstreamsEndpoint;
use crate::substreams_stream::{BlockResponse, SubstreamsStream};
use crate::pb::database::{DatabaseChanges, TableChange};
use crate::pb::database::table_change::Operation;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let endpoint_url = env::args().nth(1).expect("please provide a <endpoint_url>");
    let package_file = env::args().nth(2).expect("please provide a <package_file>");
    let module_name = env::args().nth(3).expect("please provide a <module_name>");
    let database = database_connect().await?.database("testing");

    let token_env = env::var("SUBSTREAMS_API_TOKEN").expect("please set env with: SUBSTREAMS_API_TOKEN");
    let mut token: Option<String> = None;
    if token_env.len() > 0 {
        token = Some(token_env);
    }
    info!("> Staring!");
    info!("endpoint_url={:?}\npackage_file{:?}\nmodule_name={:?}", endpoint_url, &package_file, &module_name);

    let package = read_package(&package_file)?;
    let endpoint = Arc::new(SubstreamsEndpoint::new(&endpoint_url, token).await?);


    let cursor = database_cursor_get(database.clone(), module_name.clone()).await;
    info!("cursor={:?}", cursor.clone());

    let mut stream = SubstreamsStream::new(
        endpoint.clone(),
        cursor.clone(),
        package.modules.clone(),
        module_name.clone().to_string(),
        179432144,
        179433145,
    );

    info!("> Setup completed!");
    loop {
        match stream.next().await {
            None => {
                println!("Stream consumed");
                break;
            }
            Some(event) => match event {
                Err(_) => {
                    println!("Error");
                }

                Ok(BlockResponse::New(data)) => {
                    //TODO: do something with this cursor!
                    let cursor = Some(data.cursor.clone());
                    database_cursor_create(database.clone(), module_name.clone().to_string(), cursor.clone()).await?;

                    println!("Consuming module output (cursor {})", data.cursor);

                    match extract_database_changes_from_map(data, &module_name) {
                        Ok(DatabaseChanges { table_changes }) => {
                            for table_changed in table_changes {
                                match table_changed.operation() {
                                    Operation::Unset => {
                                        warn!("operation not supported")
                                    }
                                    Operation::Create => {
                                        info!("DB-Create");
                                        database_create(database.clone(), table_changed).await?
                                    }
                                    Operation::Update => {
                                        warn!("operation not supported")
                                    }
                                    Operation::Delete => {
                                        warn!("operation not supported")
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            error!("not correct module");
                        }
                    }
                }
            },
        }
    }
    Ok(())
}

pub fn decode<T: std::default::Default + prost::Message>(buf: &Vec<u8>) -> Result<T, DecodeError> {
    ::prost::Message::decode(&buf[..])
}

fn read_package(file: &str) -> Result<Package, anyhow::Error> {
    use prost::Message;
    let content = std::fs::read(file).context(format_err!("read package {}", file))?;
    Package::decode(content.as_ref()).context("decode command")
}


fn extract_database_changes_from_map(data: BlockScopedData, module_name: &String) -> Result<DatabaseChanges, Error> {
    let output = data
        .outputs
        .first()
        .ok_or(format_err!("expecting one module output"))?;
    if &output.name != module_name {
        return Err(format_err!(
            "invalid module output name {}, expecting {}",
            output.name,
            module_name
        ));
    }

    match output.data.as_ref().unwrap() {
        MapOutput(data) => {
            let wrapper: DatabaseChanges = Message::decode(data.value.as_slice())?;
            Ok(wrapper)
        }
        _ => {
            Err(format_err!("invalid module output StoreDeltas, expecting MapOutput"))
        }
    }
}

