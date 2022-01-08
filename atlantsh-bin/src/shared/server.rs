use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Result};
use serde_json::Value;

use atlantsh_interface::server::{Request, Response};
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufReader, ErrorKind, Read, Write};
use std::path::PathBuf;
use std::time::UNIX_EPOCH;
use std::{io, process};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerState {
    pid: u32,
    working_dir: PathBuf,
    creation_time: u64,
}

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

pub struct ServerInstance<In: Write, Out: Read> {
    state: ServerState,
    server_stdin: In,
    server_output: Out,
}

impl<In: Write, Out: Read> Drop for ServerInstance<In, Out> {
    fn drop(&mut self) {
        self._quit();
    }
}

impl<In: Write, Out: Read> ServerInstance<In, Out> {
    pub fn new(state: ServerState, server_stdin: In, server_output: Out) -> Self {
        ServerInstance {
            state,
            server_stdin,
            server_output,
        }
    }
    pub fn state(&self) -> &ServerState {
        &self.state
    }

    pub fn send_message(&mut self, msg: impl Into<Request>) -> Result<Response> {
        let message = msg.into();

        serde_json::to_writer(&mut self.server_stdin, &message);
        self.server_stdin.flush();
        let ref mut reader = self.server_output;

        let de = serde_json::Deserializer::from_reader(reader);
        let mut deserializer = de.into_iter();
        match deserializer.next() {
            Some(r) => r.map_err(|e| e.into()),
            None => Err(anyhow!("Server closed")),
        }
    }

    fn _quit(&mut self) -> Result<()> {
        self.send_message("quit".to_string()).map(|_| ())
    }
}
