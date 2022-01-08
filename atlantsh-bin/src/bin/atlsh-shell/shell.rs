use std::collections::HashMap;
use std::path::PathBuf;
use atlantsh_bin::shared::ServerState;
use crate::job::Job;

pub type JobIdT = usize;

pub struct AntlanshInstance {
    cwd: PathBuf,
    env: HashMap<String, String>,
    jobs: HashMap<JobIdT, Job>,
    foreground_task: Option<JobIdT>,
    server: ServerState
}