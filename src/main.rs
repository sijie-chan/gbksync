mod git;
use git::*;

use rui::*;
use tokio::time::{interval, Duration};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let repo = open("./")?; //.expect("current repo is not a repo");
    let mut commit_interval = interval(Duration::from_secs(10));
    let mut interval_count = 0;

    loop {
        commit_interval.tick().await;
        interval_count = (interval_count + 1) % 10;
        match stage_files(&repo).ok() {
            Some(file_count) if file_count != 0 => {
                info!("staged {file_count} files");
                //.expect("stage file error");
                commit_files(&repo).ok(); //.expect("commit file failed");
            }
            _ => {}
        }
        if interval_count == 0 {
            push(&repo, "origin").ok();
        }
    }
}
