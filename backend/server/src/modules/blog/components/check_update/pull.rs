use crate::modules::blog::components::check_update::changes::{Change, DeltaIdx};
use crate::modules::blog::components::check_update::error::Error;
use crate::utils::git;
use crate::utils::git::FileDelta;
use git2::{Delta, Repository};
use markdown::MdFile;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::error;
use uuid::Uuid;

pub async fn fetch_deltas(
    git_url: &str,
    git_remote_name: &str,
    git_branch: &str,
    git_work_dir: &PathBuf,
) -> Result<Vec<FileDelta>, Error> {
    let repo = match Repository::open(git_work_dir) {
        Ok(repo) => repo,
        Err(_) => match Repository::clone(git_url, git_work_dir) {
            Ok(repo) => repo,
            Err(e) => {
                error!("failed to clone blog content git({}): {}", &git_url, e);
                return Err(e.into());
            }
        },
    };

    let updates = match git::sync(git_remote_name, git_branch, &repo) {
        Ok(updates) => updates,
        Err(e) => {
            error!(
                "failed to sync blog content git({}) updates: {}",
                &git_url, e
            );
            return Err(e.into());
        }
    };

    Ok(updates)
}

pub async fn analyze_deltas(file_deltas: Vec<FileDelta>, git_work_dir: &Path) -> Vec<Change> {
    dbg!(&file_deltas);
    let mut changes: Vec<Change> = vec![];
    let mut merge_deltas: Vec<FileDelta> = vec![];

    for delta in file_deltas.into_iter() {
        if delta.status == Delta::Modified {
            let file_path = &git_work_dir.join(
                delta
                    .new_path
                    .expect("cannot parse modified post `new_path`"),
            );
            let post = match MdFile::from_file(file_path) {
                Ok(post) => post,
                Err(e) => {
                    error!("md file parsing error: {}", e);
                    continue;
                }
            };

            let id = match extract_post_id(&post) {
                Some(id) => id,
                None => {
                    // todo: add to unresolved error list
                    error!(
                        "failed to parse uuid on post {}",
                        file_path.to_string_lossy()
                    );
                    continue;
                }
            };

            changes.push(Change::UPDATE {
                uuid: id,
                content: post.content,
            })
        } else {
            merge_deltas.push(delta)
        };
    }

    // merge some addition and remove deltas to rename/move changes
    let delta_idx = {
        let mut delta_idx: HashMap<Uuid, Vec<DeltaIdx>> = HashMap::new();
        for file_delta in merge_deltas.into_iter() {
            let file_path = &git_work_dir.join(
                file_delta
                    .new_path
                    .expect("cannot parse modified post `new_path`"),
            );
            match file_delta.status {
                // todo: split impl
                Delta::Added | Delta::Deleted => {
                    let file_path_str = file_path.to_string_lossy();

                    let post = match MdFile::from_file(file_path) {
                        Ok(post) => post,
                        Err(e) => {
                            error!("md file parsing error ({}): {}", file_path_str, e);
                            continue;
                        }
                    };

                    let id = match extract_post_id(&post) {
                        Some(id) => id,
                        None => {
                            // todo: add to unresolved error list
                            error!("failed to parse uuid on post {}", file_path_str);
                            continue;
                        }
                    };

                    let file_delta_idx = DeltaIdx {
                        delta_type: file_delta.status,
                        content: post.content,
                    };
                    delta_idx
                        .entry(id)
                        // todo: change to shallow copy
                        .and_modify(|v| v.push(file_delta_idx.clone()))
                        .or_insert(vec![file_delta_idx]);
                }
                _ => {
                    error!("unsupported delta on file {}", file_path.to_string_lossy());
                    continue;
                }
            };
        }
        delta_idx
    };

    dbg!(&delta_idx);

    changes
}

fn extract_post_id(post: &MdFile) -> Option<Uuid> {
    match post.meta["uuid"].as_str() {
        Some(id) => match Uuid::parse_str(id) {
            Ok(id) => Some(id),
            Err(_) => None,
        },
        None => None,
    }
}
