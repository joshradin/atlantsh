use std::path::PathBuf;
use std::process;
use std::time::{Duration, Instant, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerState {
    pid: u32,
    working_dir: PathBuf,
    creation_time: u64,
}

pub mod server;

impl ServerState {
    pub fn new() -> Self {
        let system_time = std::time::SystemTime::now();
        let duration = system_time.duration_since(UNIX_EPOCH).unwrap();

        Self {
            pid: process::id(),
            working_dir: std::env::current_dir().unwrap(),
            creation_time: duration.as_secs(),
        }
    }
}
