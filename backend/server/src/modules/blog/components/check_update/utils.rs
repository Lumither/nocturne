use crate::constants::{
    config::general::{default_value, var_name},
    time::DEFAULT_DATE_FORMAT,
};

// use crate::modules::blog::components::check_update::index::index_file;
use chrono::{DateTime, FixedOffset, ParseResult};
use dirs::home_dir;
use tracing::warn;

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

pub fn parse_date(date_str: &str) -> ParseResult<DateTime<FixedOffset>> {
    DateTime::parse_from_str(format!("{}T120000", date_str).as_str(), DEFAULT_DATE_FORMAT)
}
