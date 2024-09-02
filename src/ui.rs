use crate::config::*;
use crate::git_service::GitService;
use rui::*;
use std::rc::Rc;
use tracing::info;

impl core::hash::Hash for Repo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

#[derive(Clone)]
pub struct AppState {
    repos: Rc<Vec<Repo>>,
    current: Option<String>,
}

pub fn left_view() {}
pub fn app_view(repos: Rc<Vec<Repo>>) -> impl View {
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
            let (repo_path, repo_status) = cx[root_state.clone()]
                .repos
                .as_ref()
                .iter()
                .filter(|&repo| repo.id == current)
                .collect::<Vec<&Repo>>()
                .first()
                .map_or(("", ""), |&repo| {
                    if repo.service.running.load(std::sync::atomic::Ordering::SeqCst) {
                        (repo.path.as_str(), "started")
                    } else {
                        (repo.path.as_str(), "stoped")
                    }
                });
            hstack((
                // Repo list
                vstack((
                    hstack((
                        text("Repos"),
                        button("Add", move |ctx| {
                            // open finder file picker
                        }),
                    )),
                    list(repos.as_ref().clone(), move |repo: &Repo| {
                        let id = repo.id.clone();
                        text(&format!("{}", &repo.service.repo_path))
                            .padding(Auto)
                            .background(rectangle().tap(move |ctx| {
                                info!("tap: {}", id);
                                let root_state = root_state.clone();
                                ctx[root_state].current = Some(id.clone());
                            }))
                    })
                    .background(rectangle().color(RED_HIGHLIGHT))
                    .padding(Auto),
                )), // Adjust the flex value to control the width ratio
                // Details view
                // Name
                // path
                // Start/Stop button
                vstack((
                    text(repo_path),
                    text(&current).padding(Auto),
                    button(repo_status, move |ctx| {
                        let _ = ctx[root_state.clone()]
                            .repos
                            .iter()
                            .filter(|&repo| repo.id == current)
                            .collect::<Vec<&Repo>>()
                            .first()
                            .map_or("", |&repo| {
                                if repo.service.running.load(std::sync::atomic::Ordering::SeqCst) {
                                    repo.service.stop();
                                } else {
                                    repo.service.start();
                                };
                                ""
                            });
                    }),
                ))
                .flex(), // Adjust the flex value to control the width ratio
            ))
            .window_title("gbksync")
        },
    )
}
