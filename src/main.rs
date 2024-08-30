mod git;
use git::*;

use rui::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = open("./")?;//.expect("current repo is not a repo");
    stage_files(&repo)?;//.expect("stage file error");
    commit_files(&repo)?;//.expect("commit file failed");
    push(&repo, "origin")?;
    Ok(())
}
