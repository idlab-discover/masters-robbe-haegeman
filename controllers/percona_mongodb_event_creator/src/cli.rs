use clap::{Parser, Subcommand};

/// Simple program to create events for the Percona MongoDB operator
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    /// Uri to connect to the MongoDB cluster
    #[arg(short = 'u', long, env = "MONGO_URI")]
    pub db_uri: String,

    /// Name of the MongoDB database
    #[arg(short = 'n', long, env = "MONGO_DBNAME", default_value = "test")]
    pub db_name: String,

    /// Max number of iterations in test
    #[arg(short = 'i', long, env = "MAX_ITER", default_value_t = 100000)]
    pub max_iter: u64,

    /// Testing modes
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Repeatedly read from the database and report the number of executions
    Read {
        /// Time between read operations (in s)
        #[arg(long, env = "READ_INTERVAL", default_value_t = 1)]
        interval: u64,

        /// Number of books to generate
        #[arg(long, env = "PREFILL_BOOK_COUNT", default_value_t = 1000)]
        prefill_book_count: u64,
    },
    /// Continuously write to the database
    Write {
        /// Time between write operations (in s)
        #[arg(long, env = "WRITE_INTERVAL", default_value_t = 1)]
        interval: u64,

        /// Number of books to insert each period
        #[arg(long, env = "BOOK_COUNT", default_value_t = 100)]
        book_count: u64,
    },
}
