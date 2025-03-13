use crate::constants::config::general::{default_value, var_name};
use macros::panic_with_log;
use std::sync::Arc;
use std::{
    env,
    path::{Path, PathBuf},
};

use crate::modules::blog::components::check_update::{
    // index::drop_index,
    // utils::{expand_path, index_files, search_md},
    utils::expand_path,
};
use crate::scheduler::task_func::AsyncTaskFunc;
use sqlx::PgPool;
use tracing::{info, warn, Level};

// mod error;
// mod index;
mod utils;

#[derive(Debug)]
struct Args {
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
        git_work_dir,
        git_url,
        git_remote_name,
        git_remote_branch,
    });

    move || Box::pin(test(arc_args.clone()))
}

async fn test(args: Arc<Args>) {
    info!("test");
    dbg!(&args);
}
