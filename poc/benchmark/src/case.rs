use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

use serde::Serialize;
use std::future::Future;
use std::hint::black_box;
use std::time::SystemTime;

#[derive(Debug, Serialize, Default)]
pub struct Case {
    pub resource_count: usize,
    pub kind_count: usize,
    pub duration_get_latest: Vec<u128>,
    pub duration_direct: Vec<u128>,
}

impl Case {
    pub fn new(resource_count: usize, kind_count: usize) -> Self {
        Self {
            resource_count,
            kind_count,
            ..Default::default()
        }
    }

    pub fn write_to_file(&self, file_path: &PathBuf, append: bool) -> io::Result<()> {
        let json = serde_json::to_string(self)?;

        let mut file = OpenOptions::new()
            .append(append)
            .create(true)
            .open(file_path)?;

        writeln!(file, "{json}")?;

        Ok(())
    }
}

pub async fn timed_assert_ok<F, T, E>(case_durations: &mut Vec<u128>, fut: F)
where
    F: Future<Output = Result<T, E>>,
{
    let start = SystemTime::now();
    let result = fut.await;
    let end = SystemTime::now();

    if let Ok(duration) = end.duration_since(start) {
        case_durations.push(duration.as_micros());
    }

    assert!(result.is_ok());
    let _ = black_box(result);
}
