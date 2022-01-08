use crate::shell::JobIdT;

pub struct Job {
    pub command: String,
    pub job_id: JobIdT,
    pub pid: u64,
}