use git2::{Commit, Repository, Error};
pub struct GitService {
    repo: Repository,
    // seconds
    interval: usize,
}

impl GitService {
    pub fn new(repo_path: &str) -> Result<Self, Error> {
        let repo = Repository::init(repo_path)?;
        Ok(GitService {
            repo,
            interval: 10,
        })
    }
    pub fn setInterval(&mut self, i: usize) -> &Self {
        self.interval = i;
        self
    }
}
