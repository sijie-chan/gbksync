use git2::{Commit, Error, Oid, Repository};
use std::{
    path::Path,
    sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread::JoinHandle,
};
use tokio::time::{interval, Duration, Interval};
use crate::git::*;

pub struct GitService {
    repo: Arc<Mutex<Repository>>,
    // seconds
    interval: usize,
    interval_count: u128,
    commit_interval: Interval,

    running: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
}

impl GitService {
    pub fn new(repo_path: &str) -> Result<Self, Error> {
        let repo = Repository::init(repo_path)?;
        Ok(GitService {
            repo: Arc::new(Mutex::new(repo)),
            interval: 10,
            interval_count: 0,
            commit_interval: interval(Duration::from_secs(10)),
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        })
    }
    pub fn setInterval(&mut self, i: usize) -> &Self {
        self.interval = i;
        // TODO update
        self
    }
    pub fn start(&mut self) {
        let repo = Arc::clone(&self.repo);
        let running = Arc::clone(&self.running);

        self.running.store(true, Ordering::SeqCst);
        // Lock
        // use thread
        let handle = std::thread::spawn(move || {
            let repo = repo.lock().unwrap();

            while running.load(Ordering::SeqCst) {
                stage_files(&repo).unwrap();
                commit_files(&repo).unwrap();
                push(&repo, "origin").unwrap();
            }
            
        });
    }
    pub fn stop(&self) {}


}
