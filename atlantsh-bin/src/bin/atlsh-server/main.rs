use atlantsh::shared::server::ServerState;
use atlantsh_interface::server::{Request, Response};
use clap::Parser;
use lazy_static::lazy_static;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
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

    let mut de = serde_json::Deserializer::from_reader(stdin());
    let mut de_iterator = de.into_iter::<Request>();
    while let Some(Ok(request)) = de_iterator.next() {
        // eprintln!("Received command: {:#?}", request);

        let line = request.command;

        if line == "quit" {
            break;
        } else if line == "name" {
            let response = Response::from("josh!\nradin!");
            // eprintln!("Created response: {:#?}", response);
            serde_json::to_writer(stdout(), &response);
            stdout().flush();
        } else {
            // eprintln!("Didn't receive a quit")
        }
    }
    // eprintln!("Exiting server!")
}
