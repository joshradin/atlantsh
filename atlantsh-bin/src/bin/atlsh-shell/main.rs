use atlantsh_bin::launch_command;
use atlantsh_bin::shared::ServerState;
use clap::Parser;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

mod job;
mod server_instance;
mod shell;

#[derive(Parser)]
struct Args {
    /// The working directory to run the command in.
    #[clap(short = 'D', long = "working-dir")]
    dir: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    let working_dir = args
        .dir
        .map(|p| PathBuf::from(p))
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let (mut server_in, server_out) = launch_command(
        "server",
        &[
            "--working-dir".to_string(),
            working_dir.to_str().unwrap().to_string(),
        ],
        std::env::current_dir().ok(),
    )
    .unwrap();

    let mut buffered_reader = BufReader::new(server_out);
    let mut line = String::new();

    buffered_reader
        .read_line(&mut line)
        .expect("server failed unexpectedly");
    let server_state: ServerState =
        serde_json::from_str(line.as_str()).expect("Didnt receive a server state");

    println!("received server state: {:#?}", server_state);

    writeln!(server_in, "quit").unwrap();
}

fn cleanup() {}
