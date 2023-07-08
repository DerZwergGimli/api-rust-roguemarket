use std::env;
use std::sync::Arc;

use futures::StreamExt;
use log::{error, warn};

use crate::helper::{extract_database_changes_from_map, request_token};
use crate::pb::database::DatabaseChanges;
use crate::pb::database::table_change::Operation;
use crate::read_package;
use crate::substreams::SubstreamsEndpoint;
use crate::substreams_stream::{BlockResponse, SubstreamsStream};

mod test_map01;
mod base;

async fn test_helper_substreams(expected_data: String, start: i64, stop: u64) {
    const ENDPOINT_URL: &str = "https://mainnet.sol.streamingfast.io:443";
    const PACKAGE_PATH: &str = "substreams.spkg";
    const MODULE_NAME: &str = "db_sa_trades";

    let token: Option<String> = request_token(env::var("STREAMINGFAST_KEY").expect("please set env with: STREAMINGFAST_KEY")).await;
    let endpoint = Arc::new(SubstreamsEndpoint::new(ENDPOINT_URL, token).await.unwrap());

    let package_store = read_package(PACKAGE_PATH.to_string()).expect("Error reading package file!");

    let mut stream = SubstreamsStream::new(
        endpoint.clone(),
        None,
        package_store.modules,
        MODULE_NAME.to_string(),
        start,
        stop,
    );


    loop {
        match stream.next().await {
            None => {
                break;
            }
            Some(event) => match event {
                Err(_) => {
                    println!("Error");
                    panic!("Error while handling stream?");
                }
                Ok(BlockResponse::New(data)) => {
                    let _cursor = Some(data.cursor.clone());
                    let _current_block = 0;
                    match extract_database_changes_from_map(data.clone(), MODULE_NAME.to_string()) {
                        Ok(DatabaseChanges { table_changes }) => {
                            for table_changed in table_changes {
                                match table_changed.operation() {
                                    Operation::Unspecified => {
                                        warn!("operation not supported")
                                    }
                                    Operation::Create => {
                                        // println!("{:?}", table_changed);
                                        let a = format!("{:?}", table_changed.fields);
                                        assert_eq!(a, expected_data)
                                    }
                                    Operation::Update => {
                                        warn!("operation not supported")
                                    }
                                    Operation::Delete => {
                                        warn!("operation not supported")
                                    }
                                }
                            }
                            //Update cursor
                        }
                        Err(_error) => {
                            error!("not correct module");
                        }
                    }
                }
            },
        }
    }
}