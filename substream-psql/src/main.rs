mod helper;
mod pb;

use std::env;
use std::sync::Arc;
use anyhow::{Context, format_err};
use futures::FutureExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::info;
use staratlas::symbolstore;
use staratlas::symbolstore::BuilderSymbolStore;
use tokio::task::{JoinHandle, JoinSet};
use tokio::time::{sleep, Duration};
use structopt::StructOpt;
use crate::helper::request_token;
use crate::pb::substreams::Package;

#[derive(Debug, StructOpt)]
struct Config {
    #[structopt(short = "e", long = "endpoint-url")]
    endpoint_url: String,
    #[structopt(long = "psql-url")]
    psql_url: String,
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

async fn async_task(id: usize, shared: Arc<Vec<u32>>) {
    let pb = ProgressBar::new(10);
    pb.set_prefix(format!("Thread {}", id));
    for i in 0..10 {
        pb.inc(1);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    pb.finish_with_message("done");
}

const ITEMS: u64 = 10;
const MAX_CONCURRENT: usize = 10;
const STEPS: u64 = 100;

#[tokio::main]
async fn main() {
    //Start-up and init
    env_logger::init();
    let config = Config::from_args();
    info!("Config:\n {:?}", config);

    let symbol_store = BuilderSymbolStore::new().init().await;
    let package_store = read_package(config.package_file);
    let mut token: Option<String> = request_token(env::var("STREAMINGFAST_KEY").expect("please set env with: STREAMINGFAST_KEY")).await;

    let mut block_ranges = vec![];
    if config.stop_block > 0 {
        block_ranges = generate_block_ranges(config.start_block as u64, config.stop_block, config.threads_count);
        info!("Block ranges are: {:?}", block_ranges)
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
        let pb_task = multi_pg.insert_before(&pb_main, ProgressBar::new(range[1] - range[0]));
        pb_task.set_style(pb_style.clone());

        // spawns a background task immediatly no matter if the future is awaited
        // https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html#method.spawn
        set.spawn(do_stuff(range.clone(), index, STEPS, pb_task));

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

async fn do_stuff(range: Vec<u64>, index: usize, steps: u64, pb_task: ProgressBar) -> usize {
    // set a {msg} for the task progress bar, appears right next to the progress indicator
    pb_task.set_message(format!("Creating Task_{} with range {}:{}", index, range[0], range[1]));

    // we create a loop with sleep to simulate download progress
    // using rand with a range (in millisecs) to create "download duration"
    // calculate "tick size" for each progress bar step "download duration" / "# of steps in pb_task"
    //let num = rand::thread_rng().gen_range(steps..=5000);


    // heavy downloading ...
    for _ in range[0]..range[1] {
        sleep(Duration::from_millis(10)).await;
        pb_task.inc(1);
    }
    // finish the task progress bar with a message
    pb_task.finish_with_message(format!("DONE Task_{} with range {}:{}", index, range[0], range[1]));

    index
}

fn read_package(file: String) -> Result<Package, anyhow::Error> {
    use prost::Message;
    let content = std::fs::read(file.clone()).context(format_err!("read package {}", file))?;
    info!("Package-File read!");
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