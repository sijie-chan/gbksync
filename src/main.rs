mod config;
mod git;
mod git_service;
mod network;
mod ui;
mod ui_gpui;

use config::*;
use git_service::GitService;
// use rui::*;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::{fs::File, vec};
use ui::*;
use gpui::*;
use ui_gpui::*;

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
    let _config = AppConfig::init();
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

    // let app_view = app_view(config);
    // rui::rui(app_view);
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(600.0), px(400.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Transparent,
                focus: true,
                show: true,
                titlebar: Some(TitlebarOptions {
                    title: Some("GBKSync".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                window_min_size: Some(size(px(300.0), px(300.0))),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| AppView {
                    repos: Rc::new(vec![]),
                    current: None,
                })
            },
        )
        .unwrap();
    });

    return Ok(());
}
