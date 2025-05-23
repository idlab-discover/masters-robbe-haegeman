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
    /// Number of distinct kinds to use (maximum of 5)
    /// Kinds used in order: [secret, pod, service, configmap, deployment]
    #[arg(short, long, value_parser = value_parser!(u64).range(..=5))]
    pub kind_count: u64,

    /// Number of iterations to test
    #[arg(short, long, default_value_t = 100)]
    pub iterations: usize,

    /// Do not remove the primary and its secondaries at the end
    #[arg(long)]
    pub keep_resources: bool,

    /// Append the results to the file instead of overwriting
    #[arg(short, long)]
    pub overwrite: bool,

    /// Namespace to use
    #[arg(short, long, default_value = "poc-testing")]
    pub namespace: String,

    /// Delay in seconds to wait after dummy resource creation
    #[arg(short, long, default_value_t = 0)]
    pub delay: u64,
}
