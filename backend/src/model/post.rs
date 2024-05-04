use std::{env, fs};
use std::error::Error;
use std::process::Command;
use std::str;
use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime, Utc};
use uuid::Uuid;

use crate::markdown::meta::parse_meta;
use crate::model::Post;

impl Post {
    pub fn from_path(path: String) -> Result<Self, Box<dyn Error>> {
        let md_content = fs::read_to_string(&path)?;
        let meta = parse_meta(&md_content);

        let post_id = match meta.get("id") {
            None => { Uuid::new_v4() }
            Some(id) => { Uuid::from_str(id.as_str()).expect("Invalid UUID String") }
        };

        let title = match meta.get("title") {
            None => "Title Not Found".to_string(),
            Some(title) => title.to_owned(),
        };
        let summary = match meta.get("summary") {
            None => "".to_string(),
            Some(summary) => summary.to_owned(),
        };

        let git_output = Command::new("git")
            .arg("--no-pager")
            .arg("log")
            .arg("-1")
            .arg("--format=%cI")
            .arg("--")
            .current_dir(env::var("GIT_WORK_DIR")?)
            .arg(&path)
            .output()
            .expect("Failed to execute git command");

        let last_update = if git_output.status.success() {
            NaiveDateTime::parse_from_str(
                str::from_utf8(&git_output.stdout).unwrap().trim_end(),
                "%Y-%m-%dT%H:%M:%S%z",
            )
                .unwrap_or_else(|_| Utc::now().naive_utc())
        } else {
            Utc::now().naive_utc()
        };

        let first_update: NaiveDateTime = match meta.get("date") {
            None => Utc::now().naive_utc(),
            Some(date) => NaiveDateTime::from(
                NaiveDate::parse_from_str(date, "%Y-%m-%d%z")
                    .expect("Failed to parse first_update time"),
            ),
        };

        Ok(Post {
            post_id,
            title,
            summary,
            content: md_content,
            last_update,
            first_update,
        })
    }
}
