use std::path::{Path, PathBuf};
use std::process::exit;
use clap::Parser;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    /// The working directory to run the command in.
    #[clap(short='D', long="working-dir")]
    dir: Option<String>,
    /// The atlsh command to run
    #[clap(default_value = "shell")]
    command: String,
    /// The arguments to pass off to the command
    #[clap(name="args")]
    command_args: Vec<String>
}


fn main() {
    let args: Args = Args::parse();

    let command = &*args.command;
    let command_args = args.command_args.as_slice();

    let mut working_dir = PathBuf::new();
    match args.dir {
        None => {
            working_dir.push(std::env::current_dir()?);
        }
        Some(s) => {
            working_dir.push(s);
        }
    }

    match run_command(command, command_args, working_dir.as_path()) {
        Ok(o) => {
            exit(o)
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(-1);
        }
    }
}

fn run_command(command: &str, command_args: &[String], working_dir: &Path) -> Result<i32, String> {
    let as_atlsh_command = PathBuf::from(format!("atlsh-{}", command));
    
}