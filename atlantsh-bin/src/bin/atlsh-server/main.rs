use atlantsh_bin::shared::ServerState;
use clap::Parser;
use lazy_static::lazy_static;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// The working directory to run the command in.
    #[clap(short = 'D', long = "working-dir")]
    dir: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    let working_dir: PathBuf = args
        .dir
        .map(|p| PathBuf::from(p))
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    std::env::set_current_dir(working_dir).expect("couldn't set working directory");

    let state = ServerState::new();

    let state_as_json = serde_json::to_string(&state).unwrap();

    println!("{}", state_as_json);

    let mut reader = BufReader::new(stdin());
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if line.trim() == "quit" {
            break;
        } else {
            // eprintln!("Didn't receive a quit")
        }
    }
    eprintln!("Exiting server!")
}
