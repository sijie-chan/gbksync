use crate::git::*;
use git2::{Error, Repository};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex, RwLock,
    },
    thread::JoinHandle,
};
use tokio::time::Duration;
use tracing::{error, info};

pub struct GitService {
    repo: Arc<Mutex<Repository>>,
    // seconds
    interval: Arc<AtomicU64>,
    interval_count: Arc<AtomicU64>,

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
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: Arc::new(RwLock::new(None)),
        })
    }
    pub fn set_interval(&mut self, i: u64) -> &Self {
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
                interval_count
                    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| Some((v + 1)))
                    .ok();

                
                let should_push = interval_count.load(Ordering::SeqCst) % 10 == 0;
                
                info!("interval_count: {}, should_push: {}", interval_count.load(Ordering::SeqCst), should_push);
                
                info!("starting stage files");
                match stage_files(&repo) {
                    Ok(file_count) if file_count > 0 => {
                        info!("staged {} files", file_count);
                        info!("starting commit files");
                        if let Ok(_) = commit_files(&repo) {
                            info!("committed files");
                        }
                        if should_push {
                            if let Ok(_) = push(&repo, "origin") {
                                info!("pushed files");
                            }
                        }
                    }
                    Ok(_) => info!("no files to stage"),
                    Err(e) => error!("failed to stage files: {}", e),
                }

                std::thread::sleep(Duration::from_secs(interval.load(Ordering::SeqCst)))
            }
        });
        if let Ok(mut thread_handle) = self.thread_handle.write() {
            *thread_handle = Some(handle);
        } else {
            error!("Failed to acquire write lock for thread_handle");
        }
    }
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);

        if let Ok(mut thread_handle) = self.thread_handle.write() {
            if let Some(handle) = thread_handle.take() {
                // 在新线程中等待原线程结束
                std::thread::spawn(move || {
                    if let Err(e) = handle.join() {
                        error!("Error joining thread: {:?}", e);
                    }
                });
            }
        } else {
            error!("Failed to acquire write lock for thread_handle");
        }
    }
}
