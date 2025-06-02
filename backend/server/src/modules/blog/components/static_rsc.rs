use std::{
    env,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::{
    constants::config::general::{default_value, var_name},
    modules::blog::components::check_update::utils::expand_path,
};

use regex::Regex;
use tracing::warn;

pub static BLOG_POST_PATH_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^posts/\d{4}/[a-zA-Z0-9-]+/index\.md$").unwrap());

pub static GIT_WORK_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = if let Ok(default_work_path) = env::var(var_name::WORK_DIR) {
        expand_path(default_work_path)
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
});
