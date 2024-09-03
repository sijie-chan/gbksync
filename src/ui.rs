use crate::config::*;
use crate::git_service::GitService;
use rfd::FileDialog;
use rui::*;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use tracing::info;
use vger::Color;

impl core::hash::Hash for Repo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

#[derive(Clone)]
pub struct AppState {
    repos: Rc<Vec<Rc<Repo>>>,
    current: Option<String>,
}

fn calc_repos(config: Arc<RwLock<AppConfig>>) -> Rc<Vec<Rc<Repo>>> {
    info!("calc_repos");
    Rc::new(
        config
            .read()
            .unwrap()
            .get_repos()
            .iter()
            .map(|val| {
                info!("map of calc_repos {:?}", val);
                Rc::new(Repo {
                    path: Rc::new(val.path.to_string()),
                    service: Rc::new(GitService::new(&val.path.to_string()).unwrap()),
                    started: Cell::new(false),
                    id: val.id.to_string(),
                })
            })
            .collect::<Vec<Rc<Repo>>>(),
    )
}

pub fn app_view(config: Arc<RwLock<AppConfig>>) -> impl View {
    let config_clone = config.clone();
    let repos = calc_repos(config_clone.clone());
    state(
        move || AppState {
            repos: repos.clone(),
            current: None,
        },
        move |root_state, cx| {
            let repos = cx[root_state.clone()].repos.clone();
            let current = cx[root_state.clone()]
                .current
                .as_ref()
                .map_or("".to_string(), |v| v.to_string());
            let current_clone = current.clone();
            let (repo_path, repo_status, repo_repo) = cx[root_state.clone()]
                .repos
                .iter()
                .find(|repo| repo.id == current)
                .map(|repo| {
                    if repo
                        .service
                        .running
                        .load(std::sync::atomic::Ordering::SeqCst)
                    {
                        (repo.path.as_str(), "started", Some(repo.clone()))
                    } else {
                        (repo.path.as_str(), "stopped", Some(repo.clone()))
                    }
                })
                .unwrap_or(("", "", None));
            let config_clone = config_clone.clone();
            hstack((
                // Repo list
                vstack((
                    hstack((
                        text("Repos").color(Color::new(0., 0., 0., 1.)),
                        button("Add", move |ctx| {
                            // open finder file picker
                            // Save the selected folder to the config
                            let mut should_refresh = false;
                            if let Some(dir) = FileDialog::new().set_directory("/").pick_folder() {
                                if let Ok(config) = config_clone.write() {
                                    info!("before add to config");
                                    config.add(dir.to_string_lossy().to_string());
                                    config.save().unwrap_or(());
                                    info!("after add to config");
                                    should_refresh = true;
                                }
                            }
                            if should_refresh {
                                // refresh state
                                ctx[root_state.clone()].repos = calc_repos(config_clone.clone());
                                info!("after update state");
                            }
                        }),
                    ))
                    .padding(Auto),
                    list(repos.as_ref().clone(), move |repo| {
                        let id = repo.id.clone();
                        text(&format!(
                            "{}",
                            &repo
                                .service
                                .repo_path
                                .split("/")
                                .last()
                                .unwrap_or(&repo.service.repo_path)
                        ))
                        .color(if id.clone() == current_clone {
                            AZURE_HIGHLIGHT_DARK
                        } else {
                            Color::new(0., 0., 0., 0.8)
                        })
                        .padding(Auto)
                        .background(
                            rectangle()
                                .color(Color::new(1., 1., 1., 0.))
                                .tap(move |ctx| {
                                    info!("tap: {}", id);
                                    let root_state = root_state.clone();
                                    ctx[root_state].current = Some(id.clone());
                                }),
                        )
                        .flex()
                    })
                    // .background(rectangle().color(RED_HIGHLIGHT))
                    .padding(Auto)
                    .flex(),
                )), // Adjust the flex value to control the width ratio
                // Details view
                // Name
                // path
                // Start/Stop button
                vstack((
                    text(repo_path),
                    text(&current).padding(Auto),
                    button(repo_status, move |_ctx| {
                        if let Some(ref repo) = repo_repo {
                            if repo
                                .service
                                .running
                                .load(std::sync::atomic::Ordering::SeqCst)
                            {
                                repo.service.stop();
                            } else {
                                repo.service.start();
                            }
                        }
                    }),
                ))
                .background(rectangle().color(RED_HIGHLIGHT_DARK))
                .flex(), // Adjust the flex value to control the width ratio
            ))
            .window_title("gbksync")
            .background(rectangle().color(Color::new(0.8, 0.8, 0.8, 1.)))
            .flex()
        },
    )
}
