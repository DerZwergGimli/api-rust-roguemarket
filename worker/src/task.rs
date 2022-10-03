use crate::{LOOKING_GLASS, TRUCK};
use chrono::{DateTime, NaiveDateTime, Utc};
use console::style;
use indicatif::{MultiProgress, ProgressBar};
use mongo::mongodb::MongoDBConnection;
use solana_tools::fetcher::fetcher::Fetcher;

pub async fn executeTask(
    mode: &str,
    fetcher: &Fetcher,
    database: &MongoDBConnection,
    program_sig: &str,
    count: usize,
    mut before: Option<String>,
) -> Option<String> {
    if mode == "loop" {
        before = None
    }

    let mp = MultiProgress::new();

    println!(
        "{} {}Fetching signatures...",
        style("[1/5]").bold().dim(),
        LOOKING_GLASS
    );
    let signatures = fetcher.fetch_signatures(program_sig, Some(count), before);

    println!(
        "{} {}Fetching transactions...",
        style("[2/5]").bold().dim(),
        TRUCK
    );

    let pb = mp.add(ProgressBar::new(count as u64));

    let mut transactions = Vec::new();
    signatures.clone().into_iter().for_each(|signature| {
        transactions.push(fetcher.fetch_transaction(signature));
        pb.inc(1);
    });
    pb.finish_with_message("done!");

    println!("{} {}Filtering...", style("[3/5]").bold().dim(), TRUCK);
    let filtered_transactions = fetcher.filter_transactions_for_exchange(transactions);

    println!("{} {}Map to DB...", style("[5/5]").bold().dim(), TRUCK);
    let database_transactions = fetcher.map_transactions(&filtered_transactions);

    let written_to_db = database.insert_dbTrade(&database_transactions).await;
    let last_timestamp = signatures.last().unwrap().clone().block_time;
    let last_signatire = signatures.last().unwrap().clone().signature;
    logStatus(
        mode,
        signatures.clone().len(),
        filtered_transactions.len(),
        database_transactions.len(),
        written_to_db,
        &last_signatire,
        last_timestamp.unwrap(),
    );

    return Some(last_signatire);
}

fn logStatus(
    mode: &str,
    fetched: usize,
    filtered: usize,
    mapped: usize,
    written: usize,
    last_sign: &String,
    timestamp: i64,
) {
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

    println!(
        "mode={}\tfetched\t\t-> filtered\t-> mapped\t-> written  \
        \n\t\t{}\t\t-> {}\t\t-> {}\t\t-> {}\n\
        {} {}",
        mode, fetched, filtered, mapped, written, last_sign, newdate
    )
}
