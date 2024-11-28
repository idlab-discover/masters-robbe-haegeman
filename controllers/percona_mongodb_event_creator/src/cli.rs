use clap::Parser;

/// Simple program to create events for the Percona MongoDB operator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Uri to connect to the MongoDB cluster
    #[arg(short = 'u', long, env = "MONGO_URI")]
    pub db_uri: String,

    /// Name of the MongoDB database
    #[arg(short = 'n', long, env = "MONGO_DBNAME", default_value = "test")]
    pub db_name: String,

    /// Number of books to generate
    #[arg(short = 'c', long, env = "BOOK_COUNT", default_value_t = 1000)]
    pub book_count: u32,
}
