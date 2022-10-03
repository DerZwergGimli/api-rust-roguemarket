mod emotes;
mod task;

use crate::emotes::{LOOKING_GLASS, TRUCK};
use crate::task::executeTask;
use anyhow::Result;
use console::style;
use indicatif::{MultiProgress, ProgressBar};
use log::info;
use mongo::mongodb::MongoDBConnection;
use solana_tools::fetcher::fetcher::Fetcher;
use staratlas::symbolstore::BuilderSymbolStore;
use std::time::Duration;
use std::{env, thread};

const count: usize = 100;
const PROGRAM: &str = "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("--- Staring Worker ---");
    let mut last_signature = None;
    if env::var("LASTSIG").unwrap_or("".to_string()) != "" {
        last_signature = Some(env::var("LASTSIG").unwrap());
    }

    let database =
        MongoDBConnection::new(env::var("MONGOURL").expect("NO MONGOURL").as_str()).await;

    let store = BuilderSymbolStore::new();
    let fetcher = Fetcher::new(
        env::var("RPCCLIENT").expect("RPCCLIENT not set!").as_str(),
        store.init().await,
    );

    while true {
        last_signature = executeTask(
            env::var("MODE").unwrap_or("".to_string()).as_str(),
            &fetcher,
            &database,
            PROGRAM,
            100,
            last_signature,
        )
        .await;
        thread::sleep(Duration::from_millis(5000));
    }

    Ok(())
}
