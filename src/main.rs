mod git;
mod git_service;

use git::*;

use git_service::GitService;
use rui::*;
use std::fs::File;
use tokio::time::{interval, Duration};
use tracing::{error, info};
use tracing_subscriber::{self, fmt, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建日志文件
    let file = File::create("/tmp/gbksync.log")?;

    // install global collector configured based on RUST_LOG env var.
    // 设置订阅者
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(file))
        .with(fmt::layer().with_writer(std::io::stdout))
        .init();

    let gitService = GitService::new("/Users/apple/Projects/gbksync")?;

    let repo_path = "/Users/apple/Projects/gbksync";
    info!("open repository: {}", repo_path);
    let repo = open(repo_path).expect("current repo is not a repo");
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
                Ok(_) => {}
                Err(e) => {
                    error!("{}", e.to_string());
                }
            }
        }
    }
}
