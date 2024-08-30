mod git;
use git::*;

use rui::*;

fn main() {
    let repo = open("./").expect("current repo is not a repo");
    stage_files(&repo).expect("stage file error");
    commit_files(&repo).expect("commit file failed");
}
