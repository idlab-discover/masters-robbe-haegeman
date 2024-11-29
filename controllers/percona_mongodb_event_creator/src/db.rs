use futures_util::StreamExt;
use mongodb::{bson::doc, Client, Database};

use crate::book::{self, Book};

pub async fn get_connection(
    db_uri: &str,
    db_name: &str,
) -> Result<Database, mongodb::error::Error> {
    let client = Client::with_uri_str(db_uri).await?;
    let db = client.database(db_name);

    println!("Attempting connection to {}", db_name);
    db.run_command(doc! {"ping": 1}).await?;
    println!("Connected successfully to {}", db_name);
    Ok(db)
}

pub async fn count_all_books(db: &Database) -> u64 {
    let mut i = 0;
    let collection = db.collection::<Book>("test");

    if let Ok(mut cursor) = collection.find(doc! {}).await {
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

pub async fn insert_random_books(
    db: &Database,
    book_count: u64,
) -> Result<(), mongodb::error::Error> {
    let collection = db.collection("test");

    let mut books = vec![];
    for _ in 0..book_count {
        books.push(book::generate_book())
    }

    collection.insert_many(books).await?;
    Ok(())
}
