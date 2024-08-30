mod git;
use git::*;

use rui::*;
use tokio::time::{interval, Duration};
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let repo_path = "./";
    info!("open repository: {}", repo_path);
    let repo = open(repo_path)?; //.expect("current repo is not a repo");
    let mut commit_interval = interval(Duration::from_secs(10));
    let mut interval_count = 0;
    let mut intervals = 0;

    loop {
        info!("Interval {}", intervals);
        commit_interval.tick().await;
        interval_count = (interval_count + 1) % 10;
        intervals += 1;
        match stage_files(&repo).ok() {
            Some(file_count) if file_count != 0 => {
                info!("staged {file_count} files");
                //.expect("stage file error");
                info!("ready to commit files");
                commit_files(&repo).ok(); //.expect("commit file failed");
            }
            _ => {}
        }
        if interval_count == 0 {
            info!("ready to push commits");
            match push(&repo, "origin") {
                Ok(_) => {},
                Err(msg) => {
                    error!(msg)
                }
            }
        }
    }
}
