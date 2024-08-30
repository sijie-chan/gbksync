use git2::{Commit, Repository};
pub struct GitService {
    repo: Repository,
    // seconds
    interval: usize,
}

impl GitService {
    pub fn new(repo_path: &str) -> Result<Self, String> {
        let repo = Repository::init(dir_path)?;
        GitService {
            repo,
            interval: 10,
        }
    }
    pub fn setInterval(&mut self, i: usize) -> &Self {
        self.interval = i;
        self
    }
}
