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
use std::{fs::File, vec};
use ui::*;
use std::sync::{Arc, RwLock};

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

    let git_service = Rc::new(GitService::new("/Users/apple/Projects/gbksync")?);
    let c = git_service.clone();

    let app_view = app_view(config);
    let app = state(
        move || AppState {
            started: false,
            git_service: c.clone(),
        },
        move |root_state, cx| {
            vstack((
                cx[root_state].started.padding(Auto),
                button(
                    if cx[root_state].started {
                        "stop"
                    } else {
                        "start"
                    },
                    {
                        move |cx| {
                            cx.window_title = "gbksync".into();
                            cx[root_state].started = !cx[root_state].started;
                            if cx[root_state].started {
                                cx[root_state].git_service.start();
                            } else {
                                cx[root_state].git_service.stop();
                            }
                        }
                    },
                )
                .padding(Auto),
            ))
        },
    );
    rui::rui(app_view);

    return Ok(());
}
