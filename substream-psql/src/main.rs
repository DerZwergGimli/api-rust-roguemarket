mod helper;
mod pb;


mod substreams_stream;
mod substreams;


use std::env;
use std::sync::Arc;
use anyhow::{Context, Error, format_err};
use diesel::associations::HasTable;
use futures::FutureExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{error, info, warn};
use staratlas::symbolstore;
use staratlas::symbolstore::{BuilderSymbolStore, SymbolStore};
use tokio::task::{JoinHandle, JoinSet};
use tokio::time::{sleep, Duration};
use structopt::StructOpt;
use crate::helper::{extract_database_changes_from_map, map_trade_to_struct, request_token, TaskStates, update_task_info};

use crate::pb::database::DatabaseChanges;
use crate::pb::database::table_change::Operation;
use crate::pb::substreams::Package;

use crate::substreams::SubstreamsEndpoint;
use crate::substreams_stream::{BlockResponse, SubstreamsStream};
use diesel::prelude::*;
use tokio_stream::StreamExt;
use database_psql::connection::{create_cursor, create_or_update_trade_table, get_cursor, psql_connect, update_cursor};
use database_psql::model::Cursor;


#[derive(Debug, StructOpt)]
struct Config {
    #[structopt(short = "e", long = "endpoint-url")]
    endpoint_url: String,
    #[structopt(short = "p", long = "package-file")]
    package_file: String,
    #[structopt(short = "x", long = "module-name")]
    module_name: String,
    #[structopt(short = "t", long = "threads", default_value = "1")]
    threads_count: usize,
    #[structopt(long = "start-block")]
    start_block: i64,
    #[structopt(long = "stop-block", default_value = "0")]
    stop_block: u64,
    #[structopt(long = "database-name", default_value = "rust-substreams-db-writer")]
    database_name: String,
}


const ITEMS: u64 = 10;
const MAX_CONCURRENT: usize = 10;
const STEPS: u64 = 100;

#[tokio::main]
async fn main() {
    //Start-up and init
    env_logger::init();
    let mut config = Config::from_args();
    info!("Config:\n {:?}", config);


    let database = psql_connect();
    let symbol_store = Arc::new(BuilderSymbolStore::new().init().await);
    let mut token: Option<String> = request_token(env::var("STREAMINGFAST_KEY").expect("please set env with: STREAMINGFAST_KEY")).await;
    let endpoint = Arc::new(SubstreamsEndpoint::new(config.endpoint_url, token).await.unwrap());

    let mut block_ranges = vec![];
    if config.stop_block > 0 {
        block_ranges = generate_block_ranges(config.start_block as u64, config.stop_block, config.threads_count);
        info!("Block ranges are: {:?}", block_ranges)
    } else {
        warn!("Forcing single thread mode! - since we just sync the most recent blocks...!");
        block_ranges.push(vec![config.start_block as u64, config.stop_block]);
        config.threads_count = 1;
    }

    //Config progress bars
    println!(
        "\n Fetching substream: {} using {} threads limit is {}.\n",
        config.module_name, config.threads_count, MAX_CONCURRENT,
    );
    let pb_style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {msg} ",
    )
        .unwrap()
        .progress_chars("=>-");
    let multi_pg = MultiProgress::new();
    let pb_main = multi_pg.add(ProgressBar::new(ITEMS));
    pb_main.set_style(pb_style.clone());
    pb_main.set_message("total  ");
    pb_main.tick();

    // tokio::task::JoinSet
    // setup the JoinSet to manage the join handles for our futures
    let mut set = JoinSet::new();

    let mut last_item = false;

    for (index, range) in block_ranges.iter().enumerate() {
        if index == block_ranges.len() - 1 {
            last_item = true;
        }

        let mut pb_task;

        if range[1] > 0 {
            pb_task = multi_pg.insert_before(&pb_main, ProgressBar::new(range[1] - range[0]));
        } else {
            pb_task = multi_pg.insert_before(&pb_main, ProgressBar::new(range[0]))
        }

        pb_task.set_style(pb_style.clone());

        set.spawn(run_substream(index,
                                range.clone(),
                                config.package_file.clone(),
                                config.module_name.clone(),
                                symbol_store.clone(),
                                endpoint.clone(),
                                pb_task));

        // when limit is reached, wait until a running task finishes
        // await the future (join_next().await) and get the execution result
        // here result would be a download id(u64), as you can see in signature of do_stuff
        while set.len() >= MAX_CONCURRENT || last_item {
            match set.join_next().await {
                Some(_res) => {
                    // let foo = res.unwrap()
                    /* do something with foo */
                }
                None => {
                    break;
                }
            };
            pb_main.inc(1);
        }
    }
    pb_main.finish_with_message("All substreams finished!");
}


async fn run_substream(
    task_index: usize,
    range: Vec<u64>,
    package_name: String,
    module_name: String,
    symbol_store: Arc<SymbolStore>,
    endpoint: Arc<SubstreamsEndpoint>,
    pb_task: ProgressBar) -> usize {
    update_task_info(pb_task.clone(), task_index, TaskStates::CREATING);


    let connection = &mut psql_connect();
    let cursor_db = get_cursor(connection, format!("{}_{}_{}", module_name, range[0], range[1]));


    let mut cursor: Option<String> = None;
    if cursor_db.len() > 0 {
        cursor = cursor_db[0].value.clone();
    } else {
        let new_cursor = Cursor {
            id: format!("{}_{}_{}", module_name, range[0], range[1]),
            value: None,
            block: None,
        };
        create_cursor(connection, new_cursor);
    }

    sleep(Duration::from_millis(2000)).await;


    update_task_info(pb_task.clone(), task_index, TaskStates::INITIALIZING);

    let package_store = (read_package(package_name).expect("Error reading package file!"));
    let mut stream = SubstreamsStream::new(
        endpoint.clone(),
        cursor.clone(),
        package_store.modules,
        module_name.clone().to_string(),
        range[0] as i64,
        range[1],
    );
    sleep(Duration::from_millis(2000)).await;


    loop {
        update_task_info(pb_task.clone(), task_index, TaskStates::RUNNING);

        match stream.next().await {
            None => {
                update_task_info(pb_task.clone(), task_index, TaskStates::STREAM_CONSUMED);
                sleep(Duration::from_secs(2)).await;
                break;
            }
            Some(event) => match event {
                Err(_) => {
                    println!("Error");
                    panic!("Error while handling stream?");
                }
                Ok(BlockResponse::New(data)) => {
                    update_task_info(pb_task.clone(), task_index, TaskStates::INSERTING_DB);

                    pb_task.inc(1);

                    let cursor = Some(data.cursor.clone());
                    let current_block = 0;
                    match extract_database_changes_from_map(data.clone(), module_name.clone()) {
                        Ok(DatabaseChanges { table_changes }) => {
                            for table_changed in table_changes {
                                match table_changed.operation() {
                                    Operation::Unset => {
                                        warn!("operation not supported")
                                    }
                                    Operation::Create => {
                                        let mapped = map_trade_to_struct(table_changed, symbol_store.clone()).expect("Error unwrapping db data");
                                        let current_block = mapped.block as u64;
                                        create_or_update_trade_table(connection, mapped);

                                        if range[1] > 0 {
                                            pb_task.set_position(current_block - range[0]);
                                        } else {
                                            pb_task.set_position(current_block);
                                        }
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
                            let new_cursor = Cursor {
                                id: format!("{}_{}_{}", module_name, range[0], range[1]),
                                value: cursor.clone(),
                                block: Some(current_block as i64),
                            };
                            update_cursor(connection, format!("{}_{}_{}", module_name, range[0], range[1]), new_cursor);
                        }
                        Err(error) => {
                            error!("not correct module");
                        }
                    }
                }
            },
        }
    }


    // sleep(Duration::from_millis(env::args().nth(8).unwrap_or("0".to_string()).parse::<u64>().unwrap()));
    pb_task.finish_with_message(format!("Task_{} DONE with range {}:{}", task_index, range[0], range[1]));
    task_index
//    Ok(())
}


fn read_package(file: String) -> Result<Package, anyhow::Error> {
    use prost::Message;
    let content = std::fs::read(file.clone()).context(format_err!("read package {}", file))?;
    Package::decode(content.as_ref()).context("decode command")
}

fn generate_block_ranges(start_block: u64, stop_block: u64, thread_count: usize) -> Vec<Vec<u64>> {
    let block_count_per_thread = (stop_block - start_block) / thread_count as u64;
    let mut ranges = vec![];
    for thread_num in 0..thread_count {
        ranges.push(vec![
            start_block + block_count_per_thread * thread_num as u64,
            start_block + block_count_per_thread + block_count_per_thread * thread_num as u64,
        ])
    }
    return ranges;
}