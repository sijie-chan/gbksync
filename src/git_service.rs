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
    pub fn start(&self) {}
    pub fn stop(&self) {}

    // private fns
    fn stage_files(&self) -> Result<usize, Error> {
        let mut index = repo.index()?;
    
        // for loop add_path
        let statuses = repo.statuses(None)?;
    
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
}
