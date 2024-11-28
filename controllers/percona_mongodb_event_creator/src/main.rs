use clap::Parser;

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

    db::insert_random_books(&conn, args.book_count)
        .await
        .unwrap();

    let mut total_reads = 0;

    let now = Instant::now();
    let mut interval = time::interval(time::Duration::from_secs(1));
    for _i in 0..100000 {
        interval.tick().await;

        let nr = db::count_all_books(&conn).await;
        total_reads += nr;
        let elapsed = now.elapsed();
        println!(
            "{:?} reads per sec",
            total_reads as f64 / elapsed.as_secs_f64()
        );
    }

    println!("Hello, world!");
}
