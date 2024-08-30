use git2::{Commit, Error, Oid, Repository};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use tokio::time::{interval, Duration, Interval};

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

    // private fns
    fn stage_files(&self) -> Result<usize, Error> {
        let mut index = self.repo.index()?;

        // for loop add_path
        let statuses = self.repo.statuses(None)?;

        let mut file_count = 0;

        for file in statuses.iter() {
            let status = file.status();
            if status.is_wt_new() || status.is_wt_modified() {
                if let Some(p) = file.path() {
                    let path = Path::new(p);
                    index.add_path(path)?;
                    file_count += 1;
                }
            }
        }

        index.write()?;

        Ok(file_count)
    }

    fn commit_files(&self) -> Result<Oid, Error> {
        let mut index = self.repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;
        // let tree = repo

        let mut parents = vec![];

        if let Some(head) = self.repo.head().ok() {
            let commit = head.peel_to_commit()?;
            parents.push(commit);
        }
        let sig = self.repo.signature()?;

        // message is constructed using file_name + time

        let message = format!("Update {}", "file");

        let commit_id = self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &message,
            &tree,
            &parents.iter().collect::<Vec<&Commit>>(),
        )?;

        Ok(commit_id)
    }

    fn push(&self, remote: &str) -> Result<(), Error> {
        let remote = self.repo.find_remote(remote)?;
        println!("{:?}", remote.name());

        // remote.push(refspecs, opts)?;
        Ok(())
    }
}
