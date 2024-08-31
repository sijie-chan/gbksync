use git2::{Commit, Error, Oid, ProxyOptions, RemoteCallbacks, Repository};
use std::path::Path;
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

pub fn push(repo: &Repository, remote: &str) -> Result<(), Error> {
    let mut remote = repo.find_remote(remote)?;
    info!("{:?}", remote.name());

    let refspecs: &[&str] = &["refs/heads/main:refs/heads/main"];
    let callbacks = RemoteCallbacks::new();
    let mut proxy_opts = ProxyOptions::new();

    // 设置代理
    proxy_opts.auto();

    let mut opts = git2::PushOptions::new();
    opts.remote_callbacks(callbacks);
    opts.proxy_options(proxy_opts);

    remote.push(refspecs, Some(&mut opts))?;

    Ok(())
}
