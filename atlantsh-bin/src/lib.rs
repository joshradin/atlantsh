use std::io::{BufReader, BufWriter, Stdin, Stdout};
use std::path::{Path, PathBuf};
use std::process::{ChildStdin, ChildStdout, Command, ExitStatus, Output, Stdio};

pub const ATLSH_HOME: &str = "ATLSH_HOME";

pub mod shared;

/// Returns the directory of the atlsh home, if its set
pub fn atlsh_home() -> Option<PathBuf> {
    std::env::var(ATLSH_HOME).map(|v| PathBuf::from(v)).ok()
}

pub fn as_executable(program: &str) -> String {
    let output = format!("atlsh-{}", program);
    if cfg!(windows) {
        format!("{}.exe", output)
    } else {
        output
    }
}

/// Gets a list of possible paths to look for atlsh programs
///
/// # Arguments
///
/// * `working_dir`: An optional workign directory, if supported
///
/// returns: impl IntoIterator<Item=PathBuf, IntoIter=<unknown>>
///
fn run_paths(working_dir: Option<&Path>) -> impl IntoIterator<Item = PathBuf> {
    let mut output = vec![];
    if let Some(working_dir) = working_dir {
        output.push(working_dir.to_path_buf());
    }
    if let Some(home) = atlsh_home() {
        output.push(home);
    }
    if let Ok(path) = std::env::var("PATH") {
        for p in std::env::split_paths(&path) {
            output.push(p);
        }
    }
    // println!("run paths: {:#?}", output);
    output
}

/// Runs an ATLSH program
pub fn run_command<'a>(
    command: &str,
    command_args: &[String],
    working_dir: Option<PathBuf>,
) -> Result<i32, String> {
    run_command_status(command, command_args, working_dir)
        .map(|s| s.code().unwrap())
}
fn program_in_path(paths: impl IntoIterator<Item = PathBuf>, program: &str) -> Option<PathBuf> {
    for path in paths {
        let mut program_path = path;
        program_path.push(program);
        // print!("Checking if {:?} exists", program_path);
        if program_path.exists() {
            //println!(" -> true");
            return Some(program_path);
        } else {
            //println!(" -> false");
        }
    }
    None
}


/// Runs an ATLSH program
pub fn run_command_status<'a>(
    command: &str,
    command_args: &[String],
    working_dir: Option<PathBuf>,
) -> Result<ExitStatus, String> {
    let as_atlsh_command = as_executable(command);
    let paths = run_paths(working_dir.as_ref().map(|p| p.as_path()));

    let command = program_in_path(paths, as_atlsh_command.as_str()).ok_or(format!(
        "{} is neither in the working directory or available in $PATH",
        as_atlsh_command
    ))?;

    let mut cmd = Command::new(command);
    cmd.args(command_args);
    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }

    let mut result = cmd
        .spawn()
        .map_err(|e| e.to_string())?;

    result
        .wait()
        .map_err(|e| e.to_string())
}

/// Runs an ATLSH program
pub fn run_command_output<'a>(
    command: &str,
    command_args: &[String],
    working_dir: Option<PathBuf>,
) -> Result<Output, String> {
    let as_atlsh_command = as_executable(command);
    let paths = run_paths(working_dir.as_ref().map(|p| p.as_path()));

    let command = program_in_path(paths, as_atlsh_command.as_str()).ok_or(format!(
        "{} is neither in the working directory or available in $PATH",
        as_atlsh_command
    ))?;

    let mut cmd = Command::new(command);
    cmd.args(command_args);
    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }

    let result = cmd
        .spawn()
        .map_err(|e| e.to_string())?;

    result
        .wait_with_output()
        .map_err(|e| e.to_string())
}

pub fn launch_command<'a>(
    command: &str,
    command_args: &[String],
    working_dir: Option<PathBuf>,
) -> Result<(ChildStdin, ChildStdout), String> {
    let as_atlsh_command = as_executable(command);
    let paths = run_paths(working_dir.as_ref().map(|p| p.as_path()));

    let command = program_in_path(paths, as_atlsh_command.as_str()).ok_or(format!(
        "{} is neither in the working directory or available in $PATH",
        as_atlsh_command
    ))?;

    let mut cmd = Command::new(command);


    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());

    cmd.args(command_args);
    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }

    let mut child = cmd
        .spawn()
        .map_err(|e| e.to_string())?;

    let stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    Ok((stdin, stdout))
}
