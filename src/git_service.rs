use git2::{Commit, Repository};
pub struct GitService {
    repo: Repository,
    // seconds
    interval: usize,
}

impl GitService {
    pub fn new(repo_path: &str) -> Self {
        GitService {
            repo: Repository::init(dir_path),
            interval: 10,
        }
    }
}
