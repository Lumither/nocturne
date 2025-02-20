use std::{
    env,
    fs::remove_dir_all,
    path::{Path, PathBuf},
};

use crate::{
    constants::config::general::{default_value, var_name},
    modules::blog::cron::check_update::{
        index::drop_index,
        utils::{expand_path, get_md_file_basename, index_files, search_md},
    },
    utils::git,
};
use macros::panic_with_log;

use git2::{Delta, Repository};
use sqlx::{Pool, Postgres};
use tracing::{error, info, warn, Level};

mod error;
mod index;
mod utils;

pub fn task(db_connection: &Pool<Postgres>) {
    // init
    let git_work_dir = {
        let path = if let Ok(default_log_path) = env::var(var_name::WORK_DIR) {
            expand_path(default_log_path)
        } else {
            let fallback_path = expand_path(default_value::WORK_DIR.to_string());
            warn!(
                "unset env var `{}`, using default path: {}/blog_git",
                var_name::WORK_DIR,
                fallback_path
            );
            fallback_path
        };
        Path::new(&path).join("blog_git")
    };

    // clone & check update

    let git_url = match env::var(var_name::BLOG_GIT_URL) {
        Ok(url) => url,
        Err(e) => {
            panic_with_log!(
                Level::ERROR,
                "failed to read `{}`: {}",
                var_name::BLOG_GIT_URL,
                e
            )
        }
    };
    let git_remote_name = env::var(var_name::BLOG_GIT_REMOTE_NAME)
        .unwrap_or(default_value::BLOG_GIT_REMOTE_NAME.to_string());
    let git_remote_branch = env::var(var_name::BLOG_GIT_REMOTE_BRANCH)
        .unwrap_or(default_value::BLOG_GIT_REMOTE_BRANCH.to_string());

    match Repository::open(&git_work_dir) {
        Ok(repo) => {
            // prevent git remote url mismatch (clean cache)
            match repo.find_remote(&git_remote_name) {
                Ok(remote) => {
                    if remote.url().is_some_and(|url| url != git_url) {
                        remove_dir_all(&git_work_dir).unwrap();
                        warn!("blog content git({}) url mismatch, reinitialized", &git_url);
                        return;
                    }
                }
                Err(e) => {
                    remove_dir_all(&git_work_dir).unwrap();
                    error!(
                        "unexpected error on indexing blog content git({}), reinitialized: {}",
                        &git_url, e
                    );
                }
            }

            match git::sync(&git_remote_name, &git_remote_branch, &repo) {
                Ok(updates) => {
                    if !updates.is_empty() {
                        // incremental update
                        info!(
                            "blog content git({}) update detected: {:?}",
                            &git_url, updates
                        );

                        let mut update_md: Vec<PathBuf> = vec![];

                        for update in updates {
                            if [Delta::Deleted, Delta::Untracked].contains(&update.status) {
                                if let Some(path) = &update.old_path {
                                    if drop_index(
                                        &get_md_file_basename(
                                            path.file_name().unwrap().to_str().unwrap(),
                                        ),
                                        db_connection,
                                    )
                                    .is_ok()
                                    {
                                        info!(
                                            "blog content git({}) file `{}` is detected as deleted/untracked, index dropped",
                                            &git_url,
                                            &path.display()
                                        );
                                    } else {
                                        error!(
                                            "blog content git({}) file `{}` is detected as deleted/untracked, failed to drop index",
                                            &git_url,
                                            &path.display()
                                        )
                                    }
                                }
                            } else if let Some(path) = &update.new_path {
                                if path.components().any(|comp| comp.as_os_str() == "posts")
                                    && path.extension().is_some_and(|ext| ext == "md")
                                {
                                    update_md.push(path.to_path_buf());
                                }
                            }
                        }

                        index_files(&update_md, db_connection);
                    }
                }
                Err(e) => {
                    error!(
                        "failed to sync blog content git({}) updates: {}",
                        &git_url, e
                    );
                }
            }
        }
        Err(_) => {
            match Repository::clone(&git_url, &git_work_dir) {
                Ok(_repo) => {
                    // full repo index
                    let files = search_md(git_work_dir.join("posts"));
                    index_files(&files, db_connection);
                }
                Err(e) => {
                    error!("failed to clone blog content git({}): {}", &git_url, e);
                }
            }
        }
    };
}
