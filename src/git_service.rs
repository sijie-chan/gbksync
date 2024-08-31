use crate::git::*;
use git2::{Commit, Error, Oid, Repository};
use std::cell::Cell;
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex, RwLock,
    },
    thread::JoinHandle,
};
use tokio::time::{interval, Duration, Interval};
use tracing::info;

pub struct GitService {
    repo: Arc<Mutex<Repository>>,
    // seconds
    interval: Arc<AtomicU64>,
    interval_count: Arc<AtomicU64>,
    commit_interval: Interval,

    running: Arc<AtomicBool>,
    thread_handle: Arc<RwLock<Option<JoinHandle<()>>>>,
}

impl GitService {
    pub fn new(repo_path: &str) -> Result<Self, Error> {
        let repo = Repository::init(repo_path)?;
        Ok(GitService {
            repo: Arc::new(Mutex::new(repo)),
            interval: Arc::new(AtomicU64::new(10)),
            interval_count: Arc::new(AtomicU64::new(0)),
            commit_interval: interval(Duration::from_secs(10)),
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: Arc::new(RwLock::new(None)),
        })
    }
    pub fn setInterval(&mut self, i: u64) -> &Self {
        self.interval.store(i, Ordering::SeqCst);
        self
    }
    pub fn start(&self) {
        let repo = Arc::clone(&self.repo);
        let running = Arc::clone(&self.running);
        let interval = Arc::clone(&self.interval);
        let interval_count = Arc::clone(&self.interval_count);

        self.running.store(true, Ordering::SeqCst);
        // Lock
        // use thread
        let handle = std::thread::spawn(move || {
            let repo = repo.lock().unwrap();

            while running.load(Ordering::SeqCst) {
                info!("starting stage files");
                if let Ok(file_count) = stage_files(&repo) {
                    info!("staged {} files", file_count);
                };
                info!("starting commit files");
                if let Ok(_) = commit_files(&repo) {
                    info!("committed files");
                }
                if let Ok(_) = push(&repo, "origin") {
                    info!("pushed files");
                }
                std::thread::sleep(Duration::from_secs(interval.load(Ordering::SeqCst)))
            }
        });
        if let Ok(mut thread_handle) = self.thread_handle.write() {
            *thread_handle = Some(handle);
        } else {
            eprintln!("Failed to acquire write lock for thread_handle");
        }
    }
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);

        if let Ok(mut thread_handle) = self.thread_handle.write() {
            if let Some(handle) = thread_handle.take() {
                if let Err(e) = handle.join() {
                    eprintln!("Error joining thread: {:?}", e);
                }
            }
        } else {
            eprintln!("Failed to acquire write lock for thread_handle");
        }
    }
}
