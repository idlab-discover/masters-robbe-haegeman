use clap::Parser;
use cli::Commands;
use mongodb::Database;

use std::time::Instant;
use tokio::time;

mod book;
mod cli;
mod db;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    let conn = db::get_connection(&args.db_uri, &args.db_name)
        .await
        .unwrap();

    match args.command {
        Commands::Read {
            interval,
            prefill_book_count,
        } => {
            read_function(&conn, args.max_iter, interval, prefill_book_count).await;
        }
        Commands::Write {
            interval,
            book_count,
        } => {
            write_function(&conn, args.max_iter, interval, book_count).await;
        }
    }

    println!("Hello, world!");
}

async fn read_function(conn: &Database, max_iter: u64, interval: u64, prefill_book_count: u64) {
    db::insert_random_books(conn, prefill_book_count)
        .await
        .unwrap();

    let mut total_reads = 0;

    let now = Instant::now();
    let mut interval = time::interval(time::Duration::from_secs(interval));
    for _i in 0..max_iter {
        interval.tick().await;

        let nr = db::count_all_books(conn).await;
        total_reads += nr;
        let elapsed = now.elapsed();
        println!(
            "{:?} reads per sec",
            total_reads as f64 / elapsed.as_secs_f64()
        );
    }
}

async fn write_function(conn: &Database, max_iter: u64, interval: u64, book_count: u64) {
    let now = Instant::now();
    let mut interval = time::interval(time::Duration::from_secs(interval));
    for _i in 0..max_iter {
        interval.tick().await;

        db::insert_random_books(conn, book_count).await.unwrap();

        let elapsed = now.elapsed();
        println!("[{:?}]  Inserted {} books", elapsed, book_count);
    }
}
