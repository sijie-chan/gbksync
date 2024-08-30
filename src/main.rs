mod git;
use git::*;

use rui::*;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = open("./")?;//.expect("current repo is not a repo");
    let mut commit_interval = interval(Duration::from_secs(300));

    loop {
        commit_interval.tick().await;
        println!("make commit");
        stage_files(&repo).ok();//.expect("stage file error");
        commit_files(&repo).ok();//.expect("commit file failed");
        push(&repo, "origin").ok();
    }
}
