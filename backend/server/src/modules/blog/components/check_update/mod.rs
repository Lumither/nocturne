use std::{
    env,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    constants::config::general::{default_value, var_name},
    modules::blog::components::check_update::{
        apply::apply_deltas, pull::fetch_deltas, utils::expand_path,
    },
    scheduler::task_func::AsyncTaskFunc,
};
use macros::panic_with_log;

use sqlx::PgPool;
use tracing::{log::error, trace, warn, Level};

// mod error;
// mod index;
mod apply;
mod changes;
mod error;
mod pull;
mod utils;

#[derive(Debug)]
struct Args {
    db_connection: PgPool,
    git_work_dir: PathBuf,
    git_url: String,
    git_remote_name: String,
    git_remote_branch: String,
}

pub fn task(db_connection: PgPool) -> impl AsyncTaskFunc {
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

    let arc_args = Arc::new(Args {
        db_connection,
        git_work_dir,
        git_url,
        git_remote_name,
        git_remote_branch,
    });

    move || Box::pin(workflow(arc_args.clone()))
}

async fn workflow(args: Arc<Args>) {
    let deltas = match fetch_deltas(
        &args.git_url,
        &args.git_remote_name,
        &args.git_remote_branch,
        &args.git_work_dir,
    )
    .await
    {
        Ok(updates) => updates,
        Err(e) => {
            error!("delta parsing error: {}", e);
            return;
        }
    };

    if deltas.is_empty() {
        trace!("blog git already up-to-date");
        return;
    }

    apply_deltas(&args.db_connection, deltas).await;
}
