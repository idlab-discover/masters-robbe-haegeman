use std::{fs::OpenOptions, io, io::Write};

use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Measurement {
    pub duration_get_latest: u128,
    pub duration_direct: u128,
}

#[derive(Debug, Serialize, Default)]
pub struct Case {
    pub nr_resources: usize,
    pub nr_kinds: usize,
    pub measurements: Vec<Measurement>,
}

pub fn append_case_to_file(case: &Case, file_path: &str) -> io::Result<()> {
    // Serialize the case to a JSON string
    let json = serde_json::to_string(case)?;

    // Open the file in append mode, create it if it doesn't exist
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    // Append the JSON string followed by a newline
    writeln!(file, "{}", json)?;

    Ok(())
}
