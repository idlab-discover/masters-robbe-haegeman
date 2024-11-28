use clap::Parser;
use futures_util::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection, Database};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio::time;

/// Simple program to create events for the Percona MongoDB operator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Uri to connect to the MongoDB cluster
    #[arg(short = 'u', long, env = "MONGO_URI")]
    db_uri: String,

    /// Name of the MongoDB database
    #[arg(short = 'n', long, env = "MONGO_DBNAME", default_value = "test")]
    db_name: String,

    /// Number of books to generate
    #[arg(short = 'c', long, env = "BOOK_COUNT", default_value_t = 1000)]
    book_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let c = get_connection(&args.db_uri, &args.db_name).await.unwrap();

    generate_books(&c, args.book_count).await.unwrap();

    let mut total_reads = 0;

    let now = Instant::now();
    let mut interval = time::interval(time::Duration::from_secs(1));
    for _i in 0..100000 {
        interval.tick().await;

        let nr = read_ops(&c).await;
        total_reads += nr;
        let elapsed = now.elapsed();
        println!(
            "{:?} reads per sec",
            total_reads as f64 / elapsed.as_secs_f64()
        );
    }

    println!("Hello, world!");
}

async fn read_ops(db: &Database) -> i32 {
    let mut i = 0;
    let conn: Collection<Book> = db.collection("test");

    if let Ok(mut cursor) = conn.find(doc! {}).await {
        while let Some(result) = cursor.next().await {
            match result {
                Ok(_) => {
                    i += 1;
                }
                Err(_) => print!("err while getting next doc"),
            }
        }
    }

    i
}

async fn get_connection(db_uri: &str, db_name: &str) -> Result<Database, mongodb::error::Error> {
    let client = Client::with_uri_str(db_uri).await?;
    let db = client.database(db_name);

    println!("Attempting connection to {}", db_name);
    db.run_command(doc! {"ping": 1}).await?;
    println!("Connected successfully to {}", db_name);
    Ok(db)
}

fn generate_book() -> Document {
    let title = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let author = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    doc! {"title": title, "author":author}
}

async fn generate_books(db: &Database, book_count: u32) -> Result<(), mongodb::error::Error> {
    let conn = db.collection("test");

    let mut books = vec![];
    for _ in 0..book_count {
        books.push(generate_book())
    }

    conn.insert_many(books).await?;
    Ok(())
}
