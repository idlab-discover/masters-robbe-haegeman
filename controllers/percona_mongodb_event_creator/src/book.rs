use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
}

pub fn generate_book() -> Book {
    let title = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let author = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    Book { title, author }
}
