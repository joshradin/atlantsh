use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub struct AtlantshServer<In: io::Write, Out: io::Read> {
    input: In,
    output: Out,
    working_dir: PathBuf,
}

pub struct AtlantshServerLock<'l> {
    path: &'l Path,
}

impl<'l> AtlantshServerLock<'l> {
    fn lock_file_path(&self) -> PathBuf {
        Self::_lock_file_path(self.path)
    }

    fn _lock_file_path(dir: &Path) -> PathBuf {
        PathBuf::from_iter(&[dir, LOCK_FILE.as_ref()])
    }
}

impl<'l> Drop for AtlantshServerLock<'l> {
    fn drop(&mut self) {
        let file = self.lock_file_path();
        std::fs::remove_file(file);
    }
}

impl<In: io::Write, Out: io::Read> AtlantshServer<In, Out> {
    pub fn new(path: impl AsRef<Path>) -> Self {}

    pub fn try_lock(&mut self) -> Option<AtlantshServerLock> {
        let path = AtlantshServerLock::_lock_file_path(self.working_dir.as_path());
        if std::fs::metadata(path).is_ok() {
            None
        } else {
            Some(AtlantshServerLock {
                path: self.working_dir.as_path(),
            })
        }
    }

    /// Panics if the timeout duration is reached, but user can assume that the lock will be gotten
    /// if this function succeeds
    pub fn lock(&mut self, timeout: Duration) -> AtlantshServerLock {
        let start_time = Instant::now();
        loop {
            match self.try_lock() {
                None => {
                    if start_time.elapsed() >= timeout {
                        panic!("Timeout duration reached")
                    }
                }
                Some(out) => {
                    return out;
                }
            }
        }
    }
}

pub static INPUT_FILE: &str = ".atlsh_input";
pub static OUTPUT_FILE: &str = ".atlsh_output";
pub static LOCK_FILE: &str = ".atlsh_lock";

#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    pub command: String,
    pub args: Vec<String>,
    pub kwords: HashMap<String, Value>,
}

impl Request {
    pub fn new(command: String, args: Vec<String>, kwords: HashMap<String, Value>) -> Self {
        Request {
            command,
            args,
            kwords,
        }
    }
}

impl From<String> for Request {
    fn from(s: String) -> Self {
        Request::new(s, vec![], HashMap::new())
    }
}

impl From<&str> for Request {
    fn from(s: &str) -> Self {
        Request::from(s.to_string())
    }
}

impl Response {
    pub fn new(message: Value) -> Self {
        Response { message }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    message: Value,
}

impl<V> From<V> for Response
where
    Value: From<V>,
{
    fn from(f: V) -> Self {
        let value = Value::from(f);
        Response::new(value)
    }
}