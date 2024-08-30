use git2::{Commit, Error, Oid, Repository};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use tokio::time::{interval, Duration, Interval};
use git::*;

pub struct GitService {
    repo: Arc<Mutex<Repository>>,
    // seconds
    interval: usize,
    interval_count: u128,
    commit_interval: Interval,
}

impl GitService {
    pub fn new(repo_path: &str) -> Result<Self, Error> {
        let repo = Repository::init(repo_path)?;
        Ok(GitService {
            repo: Arc::new(Mutex::new(repo)),
            interval: 10,
            interval_count: 0,
            commit_interval: interval(Duration::from_secs(10)),
        })
    }
    pub fn setInterval(&mut self, i: usize) -> &Self {
        self.interval = i;
        // TODO update
        self
    }
    pub fn start(&self) {
        let repo = Arc::clone(&self.repo);
        // Lock
        // use thread
        std::thread::spawn(move || {
            let repo = repo.lock().unwrap();
            // self.stage_files();
            // self.commit_files();
            // self.push("origin");
        });
    }
    pub fn stop(&self) {}


}
