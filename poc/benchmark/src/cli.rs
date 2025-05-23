use std::path::PathBuf;

use clap::{Parser, value_parser};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub(crate) struct Args {
    /// File to output the JSONL results to
    #[arg(short, long)]
    pub file_path: PathBuf,

    /// Number of resources to create
    #[arg(short, long)]
    pub resource_count: usize,

    // u64 because of: https://github.com/clap-rs/clap/issues/4253
    /// Number of distinct kinds to use (maximum of 1)
    #[arg(short, long, value_parser = value_parser!(u64).range(..=1))]
    pub kind_count: u64,

    /// Number of iterations to test
    #[arg(short, long, default_value_t = 100)]
    pub iterations: usize,

    /// Remove the primary and its secondaries at the end
    #[arg(short, long, default_value_t = true)]
    pub cleanup: bool,

    /// Append the results to the file instead of overwriting
    #[arg(short, long, default_value_t = true)]
    pub append: bool,

    /// Namespace to use
    #[arg(short, long, default_value = "poc-testing")]
    pub namespace: String,
}
