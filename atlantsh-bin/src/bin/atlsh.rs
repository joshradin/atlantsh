use atlantsh_bin::{run_command};
use clap::Parser;
use std::path::PathBuf;
use std::process::{exit};

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    /// The working directory to run the command in.
    #[clap(short = 'D', long = "working-dir")]
    dir: Option<String>,
    /// The atlsh command to run
    #[clap(default_value = "shell")]
    command: String,
    /// The arguments to pass off to the command
    #[clap(name = "args")]
    command_args: Vec<String>,
}

fn main() {
    let args: Args = Args::parse();

    let command = &*args.command;
    let command_args = args.command_args.as_slice();

    let working_dir =
    match args.dir {
        None => {
           std::env::current_dir().ok()
        }
        Some(s) => {
            Some(PathBuf::from(s))
        }
    };

    match run_command(command, command_args, working_dir) {
        Ok(o) => exit(o),
        Err(e) => {
            eprintln!("{}", e);
            exit(-1);
        }
    }
}


