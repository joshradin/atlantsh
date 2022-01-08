use crate::job::Job;
use atlantsh::shared::server::ServerState;
use std::collections::HashMap;
use std::path::PathBuf;

pub type JobIdT = usize;

pub struct AntlanshInstance {
    cwd: PathBuf,
    env: HashMap<String, String>,
    jobs: HashMap<JobIdT, Job>,
    foreground_task: Option<JobIdT>,
    server: ServerState,
}
