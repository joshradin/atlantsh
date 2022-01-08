use atlantsh::launch_command;
use atlantsh::shared::server::{ServerInstance, ServerState};
use atlantsh_interface::server::Request;
use clap::Parser;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

mod job;
mod shell;

#[derive(Parser)]
struct Args {
    /// The working directory to run the command in.
    #[clap(short = 'D', long = "working-dir")]
    dir: Option<String>,
}

fn main() -> anyhow::Result<()> {
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

    let mut instance = ServerInstance::new(server_state, server_in, buffered_reader);

    let request = Request::from("name");
    let name = instance.send_message(request)?;
    println!("name: {:?}", name);

    Ok(())
}

fn cleanup() {}
