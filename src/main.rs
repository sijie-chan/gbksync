mod config;
mod git;
mod git_service;
mod network;
mod ui;

use config::*;
use git_service::GitService;
use rui::*;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::{fs::File, vec};
use ui::*;

use tracing::info;
use tracing_oslog::OsLogger;
use tracing_subscriber::{self, filter::EnvFilter, fmt, prelude::*};

#[derive(Clone)]
struct AppState {
    started: bool,
    git_service: Rc<GitService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(RwLock::new(AppConfig::init()));
    info!("config: {:?}", config);
    dbg!(&config);
    // 创建日志文件
    let file = File::create("/tmp/gbksync.log")?;

    // install global collector configured based on RUST_LOG env var.
    // 设置订阅者
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().with_writer(file))
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(OsLogger::new("online.welkin.gbksync", "default"))
        .init();

    let app_view = app_view(config);
    rui::rui(app_view);

    return Ok(());
}
