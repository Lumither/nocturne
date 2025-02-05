use dirs::home_dir;
use std::path::PathBuf;

use crate::blog::cron::check_update::index::index_file;
use chrono::{DateTime, FixedOffset, ParseResult};
use constants::config::general::{default_value, var_name};
use constants::time::DEFAULT_DATE_FORMAT;

use futures::executor::block_on;
use futures::future::join_all;
use sqlx::{Pool, Postgres};
use tokio::runtime::Runtime;
use tokio::spawn;
use tracing::{error, warn};

pub fn search_md(entry: PathBuf) -> Vec<PathBuf> {
    if entry.ends_with(".git") {
        return vec![];
    }
    if entry.is_dir() {
        if let Ok(files) = entry.read_dir() {
            files
                .flat_map(|file| {
                    let file_path = file.unwrap().path();
                    search_md(file_path)
                })
                .collect()
        } else {
            vec![]
        }
    } else if entry.extension().is_some_and(|ext| ext == "md") {
        vec![entry]
    } else {
        vec![]
    }
}

pub fn index_files(files: &[PathBuf], db_conn: &Pool<Postgres>) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let files_clone = dbg!(files).to_owned();
        let handlers = files_clone
            .into_iter()
            .map(|file: PathBuf| {
                let conn_clone = db_conn.clone();
                spawn(async move {
                    if let Err(e) = index_file(&file, &conn_clone).await {
                        error!("failed to index markdown file `{}`: {}", file.display(), e);
                    }
                })
            })
            .collect::<Vec<_>>();

        block_on(join_all(handlers));
    });
}

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

pub fn get_md_file_basename(f_name: &str) -> String {
    f_name.replace(".md", "")
}
