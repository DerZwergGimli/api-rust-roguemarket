extern crate core;

mod emotes;
mod task;

use crate::emotes::{LOOKING_GLASS, TRUCK};
use crate::task::execute_task;
use anyhow::Result;
use log::info;
use mongo::mongodb::MongoDBConnection;
use solana_tools::fetcher::fetcher::Fetcher;
use staratlas::symbolstore::BuilderSymbolStore;
use std::time::Duration;
use std::{env, thread};

const PROGRAM: &str = "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("--- Staring Worker ---");
    let mut last_signature = None;
    if env::var("LASTSIG").unwrap_or("".to_string()) != "" {
        last_signature = Some(env::var("LASTSIG").unwrap());
    }
    let mut count: usize = 100;
    if env::var("COUNT").unwrap_or("".to_string()) != "" {
        count = env::var("COUNT").unwrap().parse::<usize>().unwrap_or(100);
    }
    let database =
        MongoDBConnection::new(env::var("MONGOURL").expect("NO MONGOURL").as_str()).await;

    let store = BuilderSymbolStore::new();
    let fetcher = Fetcher::new(
        env::var("RPCCLIENT").expect("RPCCLIENT not set!").as_str(),
        store.init().await,
    );

    loop {
        last_signature = execute_task(
            env::var("MODE").unwrap_or("".to_string()).as_str(),
            &fetcher,
            &database,
            PROGRAM,
            count,
            last_signature,
        )
        .await;
        thread::sleep(Duration::from_millis(5000));
    }

    //Ok(())
}
