use std::path::Path;

use crate::constants::{
    config::general::{default_value, var_name},
    time::DEFAULT_NAIVE_DATE_FORMAT,
};

use chrono::NaiveDate;
use dirs::home_dir;
use markdown::MdFile;
use tracing::warn;
use uuid::Uuid;

pub fn expand_path(path: String) -> String {
    if path.starts_with('~') {
        home_dir()
            .map(|dir| {
                dir.join(path.strip_prefix("~/").unwrap())
                    .into_os_string()
                    .into_string()
                    .unwrap()
            })
            .unwrap_or_else(|| {
                warn!(
                    "failed to parse env var `{}`, using default value `{}`",
                    var_name::WORK_DIR,
                    default_value::WORK_DIR
                );
                default_value::WORK_DIR.to_string()
            })
    } else {
        path
    }
}

pub fn extract_post_id(post: &MdFile) -> Option<Uuid> {
    match post.meta["id"].as_str() {
        Some(id) => Uuid::parse_str(id).ok(),
        None => None,
    }
}

pub fn parse_post_identifier(path: &Path) -> Option<String> {
    if let Some(comp) = path.components().nth_back(1) {
        comp.as_os_str().to_str().map(|s| s.to_string())
    } else {
        None
    }
}

pub fn parse_naive_date_str(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, DEFAULT_NAIVE_DATE_FORMAT).ok()
}

#[cfg(test)]
mod tests {
    use crate::modules::blog::components::check_update::utils::parse_naive_date_str;
    use chrono::NaiveDate;

    #[test]
    fn test_parse_naive_date() {
        assert_eq!(
            NaiveDate::from_ymd_opt(2015, 1, 1),
            parse_naive_date_str("2015-1-01")
        );
    }
}
