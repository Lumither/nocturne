use std::{collections::HashMap, fs::remove_dir_all, path::PathBuf};

use crate::{
    modules::blog::components::check_update::{
        changes::{Change, Create, CreateDelete2UpdateSlot, Delete, Update},
        error::Error,
    },
    utils::{git, git::FileDelta},
};
use markdown::MdFile;

use git2::{Delta, Repository};
use tracing::{error, warn};
use uuid::Uuid;

pub async fn fetch_deltas(
    git_url: &str,
    git_remote_name: &str,
    git_remote_branch: &str,
    git_work_dir: &PathBuf,
) -> Result<Vec<Change>, Error> {
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

    let mut remote = match repo.find_remote(git_remote_name) {
        Ok(remote) => remote,
        Err(e) => {
            remove_dir_all(git_work_dir).unwrap();
            warn!(
                "cannot find git remote ({}), reinitializing: {}",
                git_remote_name, e
            );
            return Err(e.into());
        }
    };
    if remote.url().is_some_and(|url| url != git_url) {
        remove_dir_all(git_work_dir).unwrap();
        warn!(
            "git remote url mismatch (expected: {}, actual: {}), reinitializing",
            git_url,
            remote.url().unwrap()
        );
        return Err(Error::GitUrlMismatch);
    }

    let fetched_commit = git::do_fetch(&repo, &[git_remote_branch], &mut remote)?;
    let commit_deltas = git::calculate_diff(
        &repo,
        &repo.reference_to_annotated_commit(&repo.head()?)?,
        &fetched_commit,
    )?;

    // dbg!(&commit_deltas);
    let mut changes: Vec<Change> = vec![];
    let mut non_delete_deltas: Vec<FileDelta> = vec![];

    // process deleted deltas before merging commit to prevent "file not found" os error
    for delta in commit_deltas.into_iter() {
        // todo: limit scanning scope to post only OR extract merging logic to external component

        if delta.status == Delta::Deleted {
            let file_path = &git_work_dir.join(
                delta
                    .new_path
                    .expect("cannot parse modified post `new_path`"),
            );
            if let Some(id) = extract_post_id(&MdFile::from_file(file_path)?) {
                changes.push(Change::Delete(Delete {
                    uuid: id,
                    path: file_path.clone(),
                }))
            } else {
                error!("failed to parse uuid for file {}", file_path.display());
                // todo: add to unresolved error list
            }
        } else {
            non_delete_deltas.push(delta);
        }
    }

    git::do_merge(&repo, git_remote_branch, fetched_commit)?;

    // process adds and modifies deltas
    for delta in non_delete_deltas.into_iter() {
        let file_path = &git_work_dir.join(
            delta
                .new_path
                .expect("cannot parse modified post `new_path`"),
        );
        match delta.status {
            Delta::Added => {
                let md_file = MdFile::from_file(file_path)?;
                if let Some(id) = extract_post_id(&md_file) {
                    changes.push(Change::Create(Create {
                        uuid: id,
                        path: file_path.clone(),
                        payload: md_file,
                    }))
                } else {
                    error!("failed to parse uuid for file {}", file_path.display());
                    // todo: add to unresolved error list
                }
            }
            Delta::Modified => {
                let md_file = MdFile::from_file(file_path)?;
                if let Some(id) = extract_post_id(&md_file) {
                    changes.push(Change::Update(Update {
                        uuid: id,
                        path: file_path.clone(),
                        payload: md_file,
                    }))
                } else {
                    error!("failed to parse uuid for file {}", file_path.display());
                    // todo: add to unresolved error list
                }
            }
            Delta::Deleted => continue,
            _ => {
                error!("unsupported delta type on file {}", file_path.display());
                continue;
            }
        }
    }

    // merge adds and deletes deltas to moves
    let changes = dbg!(merge_changes(changes));

    Ok(changes)
}

fn extract_post_id(post: &MdFile) -> Option<Uuid> {
    match post.meta["uuid"].as_str() {
        Some(id) => Uuid::parse_str(id).ok(),
        None => None,
    }
}

fn merge_changes(changes: Vec<Change>) -> Vec<Change> {
    let mut slots: HashMap<Uuid, CreateDelete2UpdateSlot> = HashMap::new();
    let mut merged: Vec<Change> = Vec::new();

    for change in changes {
        match change {
            Change::Create(create) => {
                let id = create.uuid;
                slots.entry(id).or_default().create = Some(create);
            }
            Change::Delete(delete) => {
                let id = delete.uuid;
                slots.entry(id).or_default().delete = Some(delete);
            }
            _ => merged.push(change),
        }
    }

    for slot in slots.values() {
        if let Some(change) = slot.to_change() {
            merged.push(change);
        }
    }

    merged
}
