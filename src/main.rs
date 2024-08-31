mod git;
mod git_service;

use git_service::GitService;
use rui::*;
use std::fs::File;
use std::rc::Rc;
use tracing_subscriber::{self, filter::EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建日志文件
    let file = File::create("/tmp/gbksync.log")?;

    // install global collector configured based on RUST_LOG env var.
    // 设置订阅者
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().with_writer(file))
        .with(fmt::layer().with_writer(std::io::stdout))
        .init();

    let git_service = Rc::new(GitService::new("/Users/apple/Projects/gbksync")?);

    let app = state(
        || false,
        move |started, cx| {
            vstack((
                cx[started].padding(Auto),
                button(if cx[started] { "stop" } else { "start" }, {
                    let git_service = git_service.clone();
                    move |cx| {
                        cx[started] = !cx[started];
                        if cx[started] {
                            git_service.start();
                        } else {
                            git_service.stop();
                        }
                    }
                })
                .padding(Auto),
            ))
        },
    );
    rui::rui(app);

    return Ok(());
}
