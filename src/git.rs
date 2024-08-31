use crate::network::get_http_proxy;
use git2::{
    BranchType, Commit, Cred, Error, ErrorClass, ErrorCode, Oid, ProxyOptions, PushOptions,
    RemoteCallbacks, Repository,
};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub fn open(dir_path: &str) -> Result<Repository, Error> {
    Repository::init(dir_path)
}

pub fn stage_files(repo: &Repository) -> Result<usize, Error> {
    let mut index = repo.index()?;

    // for loop add_path
    let statuses = repo.statuses(None)?;

    let mut file_count = 0;

    for file in statuses.iter() {
        // let status = file.status();
        if let Some(p) = file.path() {
            info!("try staging file, path: {}", p);
            let path = Path::new(p);
            if let Ok(_) = index.add_path(path) {
                info!("staged file: {:?}", path);
                file_count += 1;
            }
        }
    }

    index.write()?;

    Ok(file_count)
}

pub fn commit_files(repo: &Repository) -> Result<Oid, Error> {
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    // let tree = repo

    let mut parents = vec![];

    if let Some(head) = repo.head().ok() {
        let commit = head.peel_to_commit()?;
        parents.push(commit);
    }
    let sig = repo.signature()?;

    // message is constructed using file_name + time

    let message = format!("Update {}", "file");

    let commit_id = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &message,
        &tree,
        &parents.iter().collect::<Vec<&Commit>>(),
    )?;

    Ok(commit_id)
}

pub fn commit_amend(repo: &Repository) -> Result<Oid, Error> {
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    // let tree = repo

    if let Some(head) = repo.head().ok() {
        let commit = head.peel_to_commit()?;

        let sig = repo.signature()?;

        // message is constructed using file_name + time

        let message = format!("Update {}", "file");

        let commit_id = commit.amend(
            Some("HEAD"),
            Some(&sig),
            Some(&sig),
            None,
            Some(&message),
            Some(&tree),
        )?;

        Ok(commit_id)
    } else {
        commit_files(repo)
    }
}

// 检查本地的 HEAD 是否和 origin/$current_branch 是同步的。方便后续决定是 amend 还是 commit
pub fn check_is_updated(repo: &Repository) -> Result<bool, Error> {
    // 获取当前分支
    let head = repo.head()?;
    let current_branch_name = head
        .shorthand()
        .ok_or_else(|| Error::from_str("Failed to get current branch name"))?;

    // 获取当前分支的最新提交
    let local_commit = head.peel_to_commit()?;

    // 获取远程分支
    let remote_branch_name = format!("origin/{}", current_branch_name);
    let remote_branch = repo.find_branch(&remote_branch_name, BranchType::Remote)?;

    // 获取远程分支的最新提交
    let remote_commit = remote_branch.get().peel_to_commit()?;

    // 比较本地和远程的提交 ID
    Ok(local_commit.id() == remote_commit.id())
}

pub fn push(repo: &Repository, remote: &str) -> Result<(), Error> {
    let mut remote = repo.find_remote(remote)?;
    info!("{:?}", remote.name());

    let refspecs: &[&str] = &["refs/heads/main:refs/heads/main"];
    let mut callbacks = RemoteCallbacks::new();

    // 设置认证回调
    // callbacks.credentials(|url, username_from_url, _allowed_types| {
    //     Cred::credential_helper(config, url, username);
    // });

    let mut proxy_opts = ProxyOptions::new();

    match get_http_proxy() {
        Some(proxy_url) => {
            proxy_opts.url(&proxy_url);
        }
        None => {
            proxy_opts.auto();
        }
    }

    let mut opts = PushOptions::new();
    opts.remote_callbacks(callbacks);
    opts.proxy_options(proxy_opts);

    info!("Starting push operation");
    match remote.push(refspecs, Some(&mut opts)) {
        Ok(_) => {
            info!("Push operation completed successfully");
            Ok(())
        }
        Err(e) => {
            info!("Push operation failed: {:?}", e);
            Err(e)
        }
    }
}

pub fn push_with_command(repo: &Repository) -> Result<(), Error> {
    let workdir = repo.workdir().ok_or_else(|| {
        Error::new(
            ErrorCode::Directory,
            ErrorClass::Repository,
            "Failed to get repository working directory",
        )
    })?;

    let remotes = repo.remotes()?;

    // info!("Pushing branch '{}' to origin", branch);
    let head = repo.head()?;
    let branch = head.shorthand().ok_or_else(|| {
        Error::new(
            ErrorCode::Ambiguous,
            ErrorClass::Repository,
            "Failed find remote",
        )
    })?;

    let mut remotes = remotes.iter();
    while let Some(Some(remote)) = remotes.next() {
        info!("pushing to {}, command: git push {} {}; current_dir: {:?}", remote, remote, branch, workdir);
        let output = Command::new("git")
            .arg("push")
            .arg(remote)
            .arg(branch)
            .current_dir(workdir)
            .output()
            .map_err(|e| Error::new(ErrorCode::GenericError, ErrorClass::Http, e.to_string()))?;
    }

    Ok(())
}
