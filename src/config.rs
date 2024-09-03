use crate::git_service::*;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
/// config file definition
///
use std::{cell::RefCell, fs, rc::Rc};
use uuid::Uuid;

const APP_NAME: &str = "gbksync";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FileRepoConfig {
    repo_path: String,
    stated: bool,
    id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FileConfig {
    repos: Vec<FileRepoConfig>,
}

impl From<&AppConfig> for FileConfig {
    fn from(app_config: &AppConfig) -> Self {
        let repos = app_config
            .repos
            .borrow()
            .iter()
            .map(|repo| FileRepoConfig {
                repo_path: repo.path.clone().as_ref().to_string(),
                stated: repo.started.get(),
                id: repo.id.clone(),
            })
            .collect();
        FileConfig { repos }
    }
}

#[derive(Clone, Debug)]
pub struct Repo {
    pub path: Rc<String>,
    pub service: Rc<GitService>,
    pub started: Cell<bool>,
    pub id: String,
}

impl Repo {}

#[derive(Default, Debug)]
pub struct AppConfig {
    repos: RefCell<Vec<Repo>>,
}

impl From<&FileConfig> for AppConfig {
    fn from(file_config: &FileConfig) -> Self {
        let repos = file_config
            .repos
            .iter()
            .map(|repo_config| {
                Repo {
                    path: Rc::new(repo_config.repo_path.clone()), // Initialize with appropriate values
                    service: Rc::new(GitService::new(&repo_config.repo_path).unwrap()), // Initialize with appropriate values
                    started: Cell::new(false),
                    id: repo_config.id.clone(), // Initialize with appropriate values
                }
            })
            .collect();
        Self {
            repos: RefCell::new(repos),
        }
    }
}

impl AppConfig {
    pub fn init() -> Self {
        // FIXME: get xdg or application support config path
        let config_path = AppConfig::get_config_path();
        let config = fs::read_to_string(config_path).unwrap_or("".into());
        let file_config = serde_json::from_str::<FileConfig>(&config).unwrap_or_default();
        let repos = file_config
            .repos
            .iter()
            .map(|repo_config| {
                Repo {
                    path: Rc::new(repo_config.repo_path.clone()), // Initialize with appropriate values
                    service: Rc::new(GitService::new(&repo_config.repo_path).unwrap()), // Initialize with appropriate values
                    started: Cell::new(false),
                    id: repo_config.id.clone(), // Initialize with appropriate values
                }
            })
            .collect();
        Self {
            repos: RefCell::new(repos),
        }
    }
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = AppConfig::get_config_path();
        let config = serde_json::to_string_pretty(&FileConfig::from(self))?;
        AppConfig::ensure_config_dir()?;
        // Open the file with write and create options
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_path)?;

        // Write the config to the file
        file.write_all(config.as_bytes())?;
        Ok(())
    }
    pub fn get_repos(&self) -> Vec<Repo> {
        self.repos.borrow().clone()
    }

    pub fn add(&self, repo_path: String) -> &Self {
        let id = Uuid::new_v4().to_string();
        self.repos.borrow_mut().push(Repo {
            path: Rc::new(repo_path.clone()),
            service: Rc::new(GitService::new(&repo_path).unwrap()),
            started: Cell::new(false),
            id,
        });
        self
    }

    fn ensure_config_dir() -> Result<(), Box<dyn std::error::Error>> {
        let config_path = AppConfig::get_config_path();
        fs::create_dir_all(config_path.parent().unwrap())?;
        Ok(())
    }

    fn get_config_path() -> PathBuf {
        env::var_os("HOME")
            .map(|home| {
                let mut path = PathBuf::from(home);
                path.push("Library");
                path.push("Application Support");
                path.push(APP_NAME);
                path.push("config.json");
                path
            })
            .unwrap_or_else(|| PathBuf::from("config.json"))
    }
}

#[cfg(test)]
mod test_app_config {
    use super::*;

    #[test]
    fn test_parse_file_config() {
        let content = r#"{"repos": []}"#;
        let config = serde_json::from_str::<FileConfig>(content).unwrap_or_default();
        assert_eq!(config.repos.len(), 0);
    }

    #[test]
    fn test_init_app_config() {
        let config = AppConfig::init();
        let result = config.save();
        println!("{:?}", result);
        assert!(result.is_ok());
        // assert_eq!(config.repos.borrow().len(), 0);
    }
}
